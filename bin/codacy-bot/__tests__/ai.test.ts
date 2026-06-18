import { describe, it, expect } from "bun:test";

describe("AI client (integration)", () => {
  it("runAiFix is exported", async () => {
    const mod = await import("../ai.ts");
    expect(mod.runAiFix).toBeDefined();
    expect(typeof mod.runAiFix).toBe("function");
  });

  it("MODEL_MAP uses correct model IDs", async () => {
    const fs = await import("node:fs/promises");
    const content = await fs.readFile(
      new URL("../ai.ts", import.meta.url).pathname,
      "utf-8",
    );
    expect(content).toContain('t1: "opencode-go/mimo-v2.5"');
    expect(content).toContain('t2: "opencode-go/deepseek-v4-flash"');
  });
});
