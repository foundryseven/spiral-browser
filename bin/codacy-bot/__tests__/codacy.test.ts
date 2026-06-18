import { describe, it, expect } from "bun:test";
import { classifyIssue, type CodacyIssue } from "../codacy";

describe("classifyIssue", () => {
  it("returns 't1' for warning severity", () => {
    const issue: CodacyIssue = {
      rule: "test-rule",
      severity: "Warning",
      category: "CodeStyle",
      filePath: "test.sh",
      line: 1,
      message: "test",
    };
    expect(classifyIssue(issue)).toBe("t1");
  });

  it("returns 't1' for info severity", () => {
    const issue: CodacyIssue = {
      rule: "test-rule",
      severity: "Info",
      category: "CodeStyle",
      filePath: "test.sh",
      line: 1,
      message: "test",
    };
    expect(classifyIssue(issue)).toBe("t1");
  });

  it("returns 't2' for error severity", () => {
    const issue: CodacyIssue = {
      rule: "test-rule",
      severity: "Error",
      category: "ErrorProne",
      filePath: "test.sh",
      line: 1,
      message: "test",
    };
    expect(classifyIssue(issue)).toBe("t2");
  });

  it("returns 't2' for critical severity", () => {
    const issue: CodacyIssue = {
      rule: "test-rule",
      severity: "Critical",
      category: "Security",
      filePath: "test.sh",
      line: 1,
      message: "test",
    };
    expect(classifyIssue(issue)).toBe("t2");
  });

  it("returns 't1' for unknown severity", () => {
    const issue: CodacyIssue = {
      rule: "test-rule",
      severity: "unknown",
      category: "CodeStyle",
      filePath: "test.sh",
      line: 1,
      message: "test",
    };
    expect(classifyIssue(issue)).toBe("t1");
  });
});
