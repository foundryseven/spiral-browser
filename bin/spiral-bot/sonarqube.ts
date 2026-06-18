export interface SonarIssue {
  rule: string;
  severity: string;
  type: string;
  filePath: string;
  line: number;
  message: string;
  effort?: string;
}

interface SonarIssueResponse {
  issues: Array<{
    rule: string;
    severity: string;
    type: string;
    component: string;
    line?: number;
    message: string;
    effort?: string;
  }>;
  paging?: {
    pageIndex: number;
    pageSize: number;
    total: number;
  };
}

const SONAR_API_BASE = "https://sonarcloud.io/api";

export async function fetchSonarIssues(
  projectKey: string,
  pullRequestNumber: number,
  apiToken: string,
): Promise<SonarIssue[]> {
  const issues: SonarIssue[] = [];
  const pageSize = 100;
  let pageIndex = 1;
  let total = Infinity;

  while (issues.length < total) {
    const url = new URL(
      `${SONAR_API_BASE}/issues/search`,
    );
    url.searchParams.set("componentKeys", projectKey);
    url.searchParams.set("pullRequest", String(pullRequestNumber));
    url.searchParams.set("resolved", "false");
    url.searchParams.set("ps", String(pageSize));
    url.searchParams.set("p", String(pageIndex));

    const res = await fetch(url.toString(), {
      headers: {
        Authorization: `Bearer ${apiToken}`,
        Accept: "application/json",
      },
    });

    if (!res.ok) {
      throw new Error(
        `SonarQube API ${res.status}: ${await res.text().catch(() => res.statusText)}`,
      );
    }

    const body: SonarIssueResponse = await res.json();

    for (const item of body.issues ?? []) {
      // component is like "projectKey:src/path/to/file.ts"
      const colonIdx = item.component.indexOf(":");
      const filePath =
        colonIdx >= 0
          ? item.component.slice(colonIdx + 1)
          : item.component;

      issues.push({
        rule: item.rule,
        severity: item.severity,
        type: item.type,
        filePath,
        line: item.line ?? 1,
        message: item.message,
        effort: item.effort,
      });
    }

    total = body.paging?.total ?? issues.length;
    pageIndex++;

    if (!body.issues || body.issues.length < pageSize) {
      break;
    }
  }

  return issues;
}

export function classifyIssue(issue: SonarIssue): "t1" | "t2" {
  const sev = issue.severity.toLowerCase();
  if (sev === "blocker" || sev === "critical" || sev === "major") {
    return "t2";
  }
  return "t1";
}
