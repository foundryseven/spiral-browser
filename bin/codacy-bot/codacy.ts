export interface CodacyIssue {
  rule: string;
  severity: string;
  category: string;
  filePath: string;
  line: number;
  message: string;
  suggestion?: string;
}

interface CodacyIssueResponse {
  data: Array<{
    pattern?: { name?: string };
    severity?: string;
    category?: string;
    filePath?: string;
    line?: number;
    message?: string;
    suggestion?: string;
  }>;
  pagination?: { cursor?: string };
}

export async function fetchCodacyIssues(
  commitSha: string,
  apiToken: string,
): Promise<CodacyIssue[]> {
  const issues: CodacyIssue[] = [];
  let cursor: string | undefined;

  do {
    const url = new URL(
      `https://app.codacy.com/api/v3/commits/${commitSha}/issues`,
    );
    if (cursor) url.searchParams.set("cursor", cursor);

    const res = await fetch(url.toString(), {
      headers: {
        "api-token": apiToken,
        "Content-Type": "application/json",
      },
    });

    if (!res.ok) {
      throw new Error(
        `Codacy API ${res.status}: ${await res.text().catch(() => res.statusText)}`,
      );
    }

    const body: CodacyIssueResponse = await res.json();

    for (const item of body.data ?? []) {
      issues.push({
        rule: item.pattern?.name ?? "unknown",
        severity: item.severity ?? "warning",
        category: item.category ?? "unknown",
        filePath: item.filePath ?? "",
        line: item.line ?? 0,
        message: item.message ?? "",
        suggestion: item.suggestion,
      });
    }

    cursor = body.pagination?.cursor;
  } while (cursor);

  return issues;
}

export function classifyIssue(issue: CodacyIssue): "t1" | "t2" {
  const sev = issue.severity.toLowerCase();
  if (sev === "error" || sev === "critical") return "t2";
  return "t1";
}
