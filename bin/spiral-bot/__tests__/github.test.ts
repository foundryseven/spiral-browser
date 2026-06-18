import { describe, it, expect } from "bun:test";
import { applyDiff } from "../github";

describe("applyDiff", () => {
  it("applies a single-hunk diff with additions", () => {
    const original = "line 1\nline 2\nline 3\nline 4\n";
    const diff = [
      "@@ -1,4 +1,5 @@",
      " line 1",
      " line 2",
      "+new line",
      " line 3",
      " line 4",
    ].join("\n");
    const result = applyDiff(original, diff);
    expect(result).toBe("line 1\nline 2\nnew line\nline 3\nline 4\n");
  });

  it("applies a single-hunk diff with replacements", () => {
    const original = "foo = 1\nbar = 2\nbaz = 3\n";
    const diff = [
      "@@ -1,3 +1,3 @@",
      " foo = 1",
      "-bar = 2",
      "+bar = 42",
      " baz = 3",
    ].join("\n");
    const result = applyDiff(original, diff);
    expect(result).toBe("foo = 1\nbar = 42\nbaz = 3\n");
  });

  it("applies a diff with offset (starts at line 5)", () => {
    const original = "aaa\nbbb\nccc\nddd\neee\nfff\nggg\n";
    const diff = [
      "@@ -5,3 +5,4 @@",
      " eee",
      "+EEE",
      " fff",
      " ggg",
    ].join("\n");
    const result = applyDiff(original, diff);
    expect(result).toBe("aaa\nbbb\nccc\nddd\neee\nEEE\nfff\nggg\n");
  });

  it("applies a diff with both additions and deletions", () => {
    const original = "alpha\nbeta\ngamma\n";
    const diff = [
      "@@ -1,3 +1,3 @@",
      " alpha",
      "-beta",
      "+BETA",
      " gamma",
    ].join("\n");
    const result = applyDiff(original, diff);
    expect(result).toBe("alpha\nBETA\ngamma\n");
  });

  it("returns null when hunks array is empty", () => {
    const result = applyDiff("foo", "--- no valid hunk ---");
    expect(result).toBeNull();
  });

  it("preserves trailing content after last hunk", () => {
    const original = "aaa\nbbb\nccc\nddd\neee\nfff\n";
    const diff = [
      "@@ -1,2 +1,3 @@",
      " aaa",
      "+AAA",
      " bbb",
    ].join("\n");
    const result = applyDiff(original, diff);
    expect(result).toBe("aaa\nAAA\nbbb\nccc\nddd\neee\nfff\n");
  });

  it("handles multiple hunks", () => {
    const original = "aaa\nbbb\nccc\nddd\neee\nfff\n";
    const diff = [
      "@@ -1,2 +1,2 @@",
      "-aaa",
      "+AAA",
      " bbb",
      "@@ -5,2 +5,2 @@",
      "-eee",
      "+EEE",
      " fff",
    ].join("\n");
    const result = applyDiff(original, diff);
    expect(result).toBe("AAA\nbbb\nccc\nddd\nEEE\nfff\n");
  });
});
