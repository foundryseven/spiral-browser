/**
 * Spiral-Bot — CI fix-bot for Spiral Browser.
 *
 * Polls Codacy API for PR findings on a 5-min cron schedule.
 * Calls OpenCode Go to draft fixes. Commits and pushes via GITHUB_TOKEN.
 * Bounded to MAX_RETRIES iterations per PR with RETRY_INTERVAL_MS gaps.
 *
 * On OpenCode Go cap hit: posts "having a rest" comment and skips.
 * On circuit-breaker: posts failure summary as a GitHub Issue.
 *
 * @see docs/methodology.md for the LLM-assisted methodology
 * @see AGENTS.md § Codacy merge gate for the bot's operating contract
 */

import { fetchCodacyIssues, classifyIssue, type CodacyIssue } from "./codacy";
import { runAiFix } from "./ai";
import { listOpenPRs, postPrComment, applyDiffAndCommit } from "./github";
import { readFile } from "node:fs/promises";
import { join } from "node:path";

// --- environment --------------------------------------------------------

const GITHUB_TOKEN = mustEnv("GITHUB_TOKEN");
const CODACY_API_TOKEN = mustEnv("CODACY_API_TOKEN");
const OPENCODE_GO_API_KEY = mustEnv("OPENCODE_GO_API_KEY");

const MAX_RETRIES = parseInt(process.env.MAX_RETRIES ?? "3", 10);
const RETRY_INTERVAL_MS = parseInt(
  process.env.RETRY_INTERVAL_MS ?? "600000",
  10,
);
const PR_NUMBER = process.env.PR_NUMBER
  ? parseInt(process.env.PR_NUMBER, 10)
  : 0;

const REPO = mustEnv("GITHUB_REPOSITORY");
const [OWNER, REPO_NAME] = REPO.split("/");

// --- helpers ------------------------------------------------------------

function mustEnv(name: string): string {
  const v = process.env[name];
  if (!v) {
    console.error(`[Spiral-Bot] Missing required env var: ${name}`);
    process.exit(1);
  }
  return v;
}

async function sleep(ms: number): Promise<void> {
  return new Promise((r) => setTimeout(r, ms));
}

async function loadPromptTemplate(): Promise<string> {
  const templatePath = join(import.meta.dir, "prompts", "codacy-fix.md");
  return readFile(templatePath, "utf-8");
}

interface PRInfo {
  number: number;
  headSha: string;
  headRef: string;
  title: string;
}

// --- GitHub helpers (specific to the bot) --------------------------------

async function fetchFileContent(
  path: string,
  ref: string,
): Promise<string | null> {
  const url = `https://api.github.com/repos/${OWNER}/${REPO_NAME}/contents/${path}?ref=${ref}`;
  const res = await fetch(url, {
    headers: {
      Authorization: `Bearer ${GITHUB_TOKEN}`,
      "X-GitHub-Api-Version": "2022-11-28",
      Accept: "application/vnd.github+json",
    },
  });

  if (res.status === 404) return null;
  if (!res.ok) return null;

  const body = await res.json();
  const encoded: string = body.content ?? "";
  return atob(encoded.replace(/\n/g, ""));
}

async function fetchPR(number: number): Promise<PRInfo> {
  const url = `https://api.github.com/repos/${OWNER}/${REPO_NAME}/pulls/${number}`;
  const res = await fetch(url, {
    headers: {
      Authorization: `Bearer ${GITHUB_TOKEN}`,
      "X-GitHub-Api-Version": "2022-11-28",
      Accept: "application/vnd.github+json",
    },
  });

  if (!res.ok) {
    throw new Error(`Failed to fetch PR #${number}: ${res.status}`);
  }

  const pr = await res.json();
  return {
    number,
    headSha: pr.head.sha,
    headRef: pr.head.ref,
    title: pr.title,
  };
}

// --- prompt building ----------------------------------------------------

function buildUserPrompt(issue: CodacyIssue, fileContent: string): string {
  return [
    `Fix the following finding:`,
    ``,
    `- Rule: ${issue.rule}`,
    `- Category: ${issue.category}`,
    `- Severity: ${issue.severity}`,
    `- File: ${issue.filePath}`,
    `- Line: ${issue.line}`,
    `- Message: ${issue.message}`,
    issue.suggestion ? `- Suggestion: ${issue.suggestion}` : "",
    ``,
    `File content:`,
    `\`\`\``,
    fileContent,
    `\`\`\``,
  ]
    .filter(Boolean)
    .join("\n");
}

// --- circuit-breaker ----------------------------------------------------

async function openCircuitBreakerIssue(
  pr: PRInfo,
  issue: CodacyIssue,
  reason: string,
): Promise<void> {
  const title = `[Spiral-Bot] Circuit-breaker: could not fix ${issue.rule} on PR #${pr.number}`;
  const body = [
    `## Spiral-Bot circuit-breaker`,
    ``,
    `Spiral-Bot exhausted ${MAX_RETRIES} attempts to fix a Codacy finding on PR #${pr.number}.`,
    ``,
    `| Field | Value |`,
    `|---|---|`,
    `| PR | #${pr.number}: "${pr.title}" |`,
    `| Commit | ${pr.headSha.slice(0, 7)} |`,
    `| Rule | ${issue.rule} |`,
    `| File | ${issue.filePath}:${issue.line} |`,
    `| Severity | ${issue.severity} |`,
    `| Reason | ${reason} |`,
    ``,
    `**Next step:** A human should review this finding and either:`,
    `1. Fix it manually and push to PR #${pr.number}.`,
    `2. Dismiss it as a false positive in Codacy.`,
    `3. Update the bot's prompt template if the fix pattern is missing.`,
    ``,
    `_Bot: Spiral-Bot · Max retries: ${MAX_RETRIES} · Interval: ${RETRY_INTERVAL_MS / 1000}s_`,
  ].join("\n");

  try {
    const url = `https://api.github.com/repos/${OWNER}/${REPO_NAME}/issues`;
    await fetch(url, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${GITHUB_TOKEN}`,
        "X-GitHub-Api-Version": "2022-11-28",
        Accept: "application/vnd.github+json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        title,
        body,
        labels: ["spiral-bot", "codacy-failure"],
      }),
    });
    console.log(`[Spiral-Bot] Circuit-breaker Issue opened.`);
  } catch (err) {
    console.error("[Spiral-Bot] Failed to open circuit-breaker Issue:", err);
  }
}

// --- core loop ----------------------------------------------------------

async function processPR(pr: PRInfo, systemPrompt: string): Promise<void> {
  console.log(
    `[Spiral-Bot] Processing PR #${pr.number}: "${pr.title}" (head: ${pr.headSha.slice(0, 7)}, branch: ${pr.headRef})`,
  );

  let issues: CodacyIssue[];
  try {
    issues = await fetchCodacyIssues(pr.headSha, CODACY_API_TOKEN);
  } catch (err) {
    const msg = String(err);
    if (msg.includes("CAP_EXCEEDED") || msg.includes("429")) {
      console.log("[Spiral-Bot] Codacy API rate limited, skipping this PR.");
      return;
    }
    console.error(`[Spiral-Bot] Codacy API error for PR #${pr.number}:`, msg);
    return;
  }

  if (issues.length === 0) {
    console.log(`[Spiral-Bot] PR #${pr.number}: no Codacy issues — skipping.`);
    return;
  }

  console.log(
    `[Spiral-Bot] PR #${pr.number}: ${issues.length} Codacy issue(s) found.`,
  );

  for (const issue of issues) {
    const tier = classifyIssue(issue);
    const modelName = tier === "t2" ? "deepseek-v4-flash" : "mimo-v2.5";
    console.log(
      `[Spiral-Bot]   → [${tier.toUpperCase()}] ${issue.rule} @ ${issue.filePath}:${issue.line} (${issue.severity})`,
    );

    // Fetch the file content once per issue
    const fileContent = await fetchFileContent(
      issue.filePath,
      pr.headRef,
    );
    if (fileContent === null) {
      console.log(
        `[Spiral-Bot]   → Could not fetch ${issue.filePath} from branch ${pr.headRef}. Skipping.`,
      );
      continue;
    }

    for (let attempt = 1; attempt <= MAX_RETRIES; attempt++) {
      console.log(
        `[Spiral-Bot]   → Attempt ${attempt}/${MAX_RETRIES} with ${modelName}`,
      );

      let result;
      try {
        result = await runAiFix(
          {
            systemPrompt,
            userPrompt: buildUserPrompt(issue, fileContent),
            tier,
          },
          OPENCODE_GO_API_KEY,
        );
      } catch (err) {
        const msg = String(err);
        if (msg.includes("CAP_EXCEEDED")) {
          await postPrComment(
            OWNER,
            REPO_NAME,
            pr.number,
            [
              `## Spiral-Bot: having a rest`,
              ``,
              `Hit the OpenCode Go usage cap while processing PR #${pr.number}.`,
              `Will resume on the next cron cycle.`,
              ``,
              `_Bot: Spiral-Bot · Model: ${modelName}_`,
            ].join("\n"),
            GITHUB_TOKEN,
          );
          console.log("[Spiral-Bot] OpenCode Go cap exceeded — skipping this PR.");
          return;
        }
        console.error(`[Spiral-Bot] AI error (attempt ${attempt}):`, msg);
        if (attempt === MAX_RETRIES) {
          await openCircuitBreakerIssue(pr, issue, msg);
        }
        if (attempt < MAX_RETRIES) {
          console.log(
            `[Spiral-Bot]   → Waiting ${RETRY_INTERVAL_MS / 1000}s before retry...`,
          );
          await sleep(RETRY_INTERVAL_MS);
        }
        continue;
      }

      if (result.cannotFixReason) {
        console.log(
          `[Spiral-Bot]   → CANNOT_FIX: ${result.cannotFixReason}`,
        );
        if (attempt === MAX_RETRIES) {
          await openCircuitBreakerIssue(
            pr,
            issue,
            `CANNOT_FIX after ${MAX_RETRIES} attempts: ${result.cannotFixReason}`,
          );
        }
        if (attempt < MAX_RETRIES) {
          console.log(
            `[Spiral-Bot]   → Waiting ${RETRY_INTERVAL_MS / 1000}s before retry...`,
          );
          await sleep(RETRY_INTERVAL_MS);
        }
        continue;
      }

      if (!result.isDiff) {
        console.log(
          `[Spiral-Bot]   → AI returned non-diff output (attempt ${attempt}).`,
        );
        if (tier === "t1") {
          // Escalate to T2 on non-diff output
          console.log("[Spiral-Bot]   → Escalating to T2...");
          try {
            result = await runAiFix(
              {
                systemPrompt,
                userPrompt: buildUserPrompt(issue, fileContent),
                tier: "t2",
              },
              OPENCODE_GO_API_KEY,
            );
            if (!result.isDiff) {
              console.log(
                "[Spiral-Bot]   → T2 also returned non-diff. Retrying...",
              );
              if (attempt < MAX_RETRIES) {
                await sleep(RETRY_INTERVAL_MS);
              }
              continue;
            }
          } catch (err) {
            console.error("[Spiral-Bot]   → T2 escalation failed:", err);
            if (attempt < MAX_RETRIES) {
              await sleep(RETRY_INTERVAL_MS);
            }
            continue;
          }
        } else {
          if (attempt < MAX_RETRIES) {
            await sleep(RETRY_INTERVAL_MS);
          }
          continue;
        }
      }

      // Apply the diff and commit
      try {
        const commitMsg = [
          `fix: ${issue.rule} in ${issue.filePath}`,
          ``,
          `Codacy finding: ${issue.severity} — ${issue.message}`,
          `File: ${issue.filePath}:${issue.line}`,
          `Attempt: ${attempt}/${MAX_RETRIES}`,
          `Model: ${result.model}`,
          ``,
          `Assisted-by: Spiral-Bot (${result.model})`,
        ].join("\n");

        const commit = await applyDiffAndCommit(
          OWNER,
          REPO_NAME,
          pr.headRef,
          issue.filePath,
          result.content,
          commitMsg,
          GITHUB_TOKEN,
        );

        console.log(
          `[Spiral-Bot]   → Committed: ${commit.sha.slice(0, 7)} (${commit.url})`,
        );

        await postPrComment(
          OWNER,
          REPO_NAME,
          pr.number,
          [
            `## Spiral-Bot: fix applied`,
            ``,
            `Fixed **${issue.rule}** in \`${issue.filePath}:${issue.line}\`.`,
            ``,
            `| Field | Value |`,
            `|---|---|`,
            `| Severity | ${issue.severity} |`,
            `| Attempt | ${attempt}/${MAX_RETRIES} |`,
            `| Model | ${result.model} |`,
            `| Tier | ${tier} |`,
            ``,
            `Commit: ${commit.sha.slice(0, 7)}`,
            ``,
            `Codacy will re-run automatically on the new commit.`,
            ``,
            `_Bot: Spiral-Bot · Model: ${result.model}_`,
          ].join("\n"),
          GITHUB_TOKEN,
        );
      } catch (err) {
        console.error(
          `[Spiral-Bot]   → GitHub API error (attempt ${attempt}):`,
          err,
        );
        if (attempt === MAX_RETRIES) {
          await openCircuitBreakerIssue(pr, issue, String(err));
        }
        if (attempt < MAX_RETRIES) {
          console.log(
            `[Spiral-Bot]   → Waiting ${RETRY_INTERVAL_MS / 1000}s before retry...`,
          );
          await sleep(RETRY_INTERVAL_MS);
        }
        continue;
      }

      break; // success — move to next issue
    }
  }
}

// --- main ---------------------------------------------------------------

async function main(): Promise<void> {
  console.log("[Spiral-Bot] Starting codacy-fix-bot run.");
  console.log(`[Spiral-Bot]   Repo: ${OWNER}/${REPO_NAME}`);
  console.log(`[Spiral-Bot]   PR: ${PR_NUMBER || "all open"}`);
  console.log(`[Spiral-Bot]   Max retries: ${MAX_RETRIES}`);
  console.log(`[Spiral-Bot]   Interval: ${RETRY_INTERVAL_MS / 1000}s`);

  const systemPrompt = await loadPromptTemplate();

  let prs: PRInfo[];

  if (PR_NUMBER > 0) {
    const pr = await fetchPR(PR_NUMBER);
    prs = [pr];
  } else {
    const rawPrs = await listOpenPRs(OWNER, REPO_NAME, GITHUB_TOKEN);
    // Fetch full PR info (including headRef) for each
    prs = await Promise.all(rawPrs.map((p) => fetchPR(p.number)));
  }

  if (prs.length === 0) {
    console.log("[Spiral-Bot] No open PRs — nothing to do.");
    return;
  }

  console.log(`[Spiral-Bot] Processing ${prs.length} PR(s).`);

  for (const pr of prs) {
    try {
      await processPR(pr, systemPrompt);
    } catch (err) {
      console.error(`[Spiral-Bot] Unhandled error on PR #${pr.number}:`, err);
    }
  }

  console.log("[Spiral-Bot] Run complete.");
}

main();
