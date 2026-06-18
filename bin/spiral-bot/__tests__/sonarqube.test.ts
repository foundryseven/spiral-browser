import { describe, it, expect } from "bun:test";
import { classifyIssue, type SonarIssue } from "../sonarqube";

describe("classifyIssue", () => {
  it("returns 't1' for minor severity", () => {
    const issue: SonarIssue = {
      rule: "rust:S3776",
      severity: "MINOR",
      type: "CODE_SMELL",
      filePath: "src/lib.rs",
      line: 1,
      message: "Reduce cognitive complexity",
    };
    expect(classifyIssue(issue)).toBe("t1");
  });

  it("returns 't1' for info severity", () => {
    const issue: SonarIssue = {
      rule: "rust:S1481",
      severity: "INFO",
      type: "CODE_SMELL",
      filePath: "src/lib.rs",
      line: 1,
      message: "Remove unused variable",
    };
    expect(classifyIssue(issue)).toBe("t1");
  });

  it("returns 't2' for major severity", () => {
    const issue: SonarIssue = {
      rule: "rust:S2245",
      severity: "MAJOR",
      type: "SECURITY_HOTSPOT",
      filePath: "src/lib.rs",
      line: 1,
      message: "Make sure that using this pseudorandom number generator is safe here",
    };
    expect(classifyIssue(issue)).toBe("t2");
  });

  it("returns 't2' for critical severity", () => {
    const issue: SonarIssue = {
      rule: "rust:S3518",
      severity: "CRITICAL",
      type: "VULNERABILITY",
      filePath: "src/lib.rs",
      line: 1,
      message: "Spotbugs: A function uses a String comparison",
    };
    expect(classifyIssue(issue)).toBe("t2");
  });

  it("returns 't2' for blocker severity", () => {
    const issue: SonarIssue = {
      rule: "rust:S4524",
      severity: "BLOCKER",
      type: "VULNERABILITY",
      filePath: "src/lib.rs",
      line: 1,
      message: "Make sure this expression is safe",
    };
    expect(classifyIssue(issue)).toBe("t2");
  });

  it("returns 't1' for unknown severity", () => {
    const issue: SonarIssue = {
      rule: "rust:UNKNOWN",
      severity: "unknown",
      type: "CODE_SMELL",
      filePath: "src/lib.rs",
      line: 1,
      message: "test",
    };
    expect(classifyIssue(issue)).toBe("t1");
  });
});
