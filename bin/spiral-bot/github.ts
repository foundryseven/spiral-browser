export interface DiffPatch {
  path: string;
  content: string;
}

export interface CommitResult {
  sha: string;
  url: string;
}

interface GitHubFile {
  sha: string;
  content: string;
}

async function getFileContent(
  owner: string,
  repo: string,
  path: string,
  ref: string,
  token: string,
): Promise<GitHubFile | null> {
  const url = `https://api.github.com/repos/${owner}/${repo}/contents/${path}?ref=${ref}`;
  const res = await fetch(url, {
    headers: {
      Authorization: `Bearer ${token}`,
      "X-GitHub-Api-Version": "2022-11-28",
      Accept: "application/vnd.github+json",
    },
  });

  if (res.status === 404) return null;
  if (!res.ok) return null;

  const body = await res.json();
  return {
    sha: body.sha,
    content: atob(body.content.replace(/\n/g, "")),
  };
}

interface Hunk {
  oldStart: number;
  oldCount: number;
  newStart: number;
  newCount: number;
  lines: string[];
}

function parseHunks(diff: string): Hunk[] {
  const hunks: Hunk[] = [];
  let current: Hunk | null = null;

  for (const line of diff.split("\n")) {
    if (line.startsWith("---") || line.startsWith("+++")) continue;

    const hunkMatch = line.match(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/);
    if (hunkMatch) {
      if (current) hunks.push(current);
      current = {
        oldStart: parseInt(hunkMatch[1], 10),
        oldCount: parseInt(hunkMatch[2] ?? "1", 10),
        newStart: parseInt(hunkMatch[3], 10),
        newCount: parseInt(hunkMatch[4] ?? "1", 10),
        lines: [],
      };
      continue;
    }

    if (current) {
      current.lines.push(line);
    }
  }

  if (current) hunks.push(current);
  return hunks;
}

export function applyDiff(originalContent: string, diffHunk: string): string | null {
  const hunks = parseHunks(diffHunk);
  if (hunks.length === 0) return null;

  const originalLines = originalContent.split("\n");
  const result: string[] = [];
  let originalIndex = 0;

  for (const hunk of hunks) {
    const startLine = hunk.oldStart - 1;

    while (originalIndex < startLine) {
      result.push(originalLines[originalIndex]);
      originalIndex++;
    }

    for (const line of hunk.lines) {
      if (line.startsWith("-")) {
        if (originalIndex < originalLines.length) {
          originalIndex++;
        }
      } else if (line.startsWith("+")) {
        result.push(line.slice(1));
      } else {
        if (originalIndex < originalLines.length) {
          result.push(originalLines[originalIndex]);
          originalIndex++;
        }
      }
    }
  }

  while (originalIndex < originalLines.length) {
    result.push(originalLines[originalIndex]);
    originalIndex++;
  }

  return result.join("\n");
}

export async function applyDiffAndCommit(
  owner: string,
  repo: string,
  branch: string,
  path: string,
  diffHunk: string,
  commitMessage: string,
  token: string,
): Promise<CommitResult> {
  const existing = await getFileContent(owner, repo, path, branch, token);
  const newContent = applyDiff(existing?.content ?? "", diffHunk);

  if (!newContent) {
    throw new Error(
      `Failed to apply diff for ${path} — diff parsing returned empty content`,
    );
  }

  const url = `https://api.github.com/repos/${owner}/${repo}/contents/${path}`;
  const body = {
    message: commitMessage,
    content: btoa(newContent),
    sha: existing?.sha,
    branch,
  };

  const res = await fetch(url, {
    method: "PUT",
    headers: {
      Authorization: `Bearer ${token}`,
      "X-GitHub-Api-Version": "2022-11-28",
      Accept: "application/vnd.github+json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    const errText = await res.text().catch(() => res.statusText);
    throw new Error(`GitHub PUT ${path}: ${res.status} — ${errText}`);
  }

  const result = await res.json();
  return {
    sha: result.commit.sha,
    url: result.commit.html_url,
  };
}

export async function postPrComment(
  owner: string,
  repo: string,
  prNumber: number,
  body: string,
  token: string,
): Promise<void> {
  const url = `https://api.github.com/repos/${owner}/${repo}/issues/${prNumber}/comments`;
  const res = await fetch(url, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "X-GitHub-Api-Version": "2022-11-28",
      Accept: "application/vnd.github+json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ body }),
  });

  if (!res.ok) {
    const errText = await res.text().catch(() => res.statusText);
    throw new Error(`GitHub POST comment: ${res.status} — ${errText}`);
  }
}

export async function listOpenPRs(
  owner: string,
  repo: string,
  token: string,
): Promise<Array<{ number: number; headSha: string; title: string }>> {
  const url = `https://api.github.com/repos/${owner}/${repo}/pulls?state=open&per_page=50`;
  const res = await fetch(url, {
    headers: {
      Authorization: `Bearer ${token}`,
      "X-GitHub-Api-Version": "2022-11-28",
      Accept: "application/vnd.github+json",
    },
  });

  if (!res.ok) {
    throw new Error(`GitHub GET pulls: ${res.status}`);
  }

  const pulls = await res.json();
  return pulls.map((p: Record<string, unknown>) => ({
    number: p.number as number,
    headSha: (p.head as Record<string, unknown>).sha as string,
    title: p.title as string,
  }));
}
