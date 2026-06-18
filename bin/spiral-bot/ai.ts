export type ModelTier = "t1" | "t2";

export interface AiRequest {
  systemPrompt: string;
  userPrompt: string;
  tier: ModelTier;
}

export interface AiResult {
  content: string;
  isDiff: boolean;
  cannotFixReason?: string;
  model: string;
  tier: ModelTier;
}

const MODEL_MAP: Record<ModelTier, string> = {
  t1: "opencode-go/mimo-v2.5",
  t2: "opencode-go/deepseek-v4-flash",
};

const API_BASE = "https://opencode.ai/zen/go/v1/chat/completions";

export async function runAiFix(
  request: AiRequest,
  apiKey: string,
): Promise<AiResult> {
  const model = MODEL_MAP[request.tier];

  const res = await fetch(API_BASE, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${apiKey}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      model,
      messages: [
        { role: "system", content: request.systemPrompt },
        { role: "user", content: request.userPrompt },
      ],
      temperature: 0.1,
      max_tokens: 2048,
    }),
  });

  if (!res.ok) {
    const errText = await res.text().catch(() => res.statusText);
    if (res.status === 429) {
      throw new Error(`CAP_EXCEEDED: OpenCode Go rate limit hit — ${errText}`);
    }
    throw new Error(`OpenCode Go ${res.status}: ${errText}`);
  }

  const body = await res.json();
  const content: string = body.choices?.[0]?.message?.content ?? "";

  const cannotMatch = content.match(
    /^CANNOT_FIX:\s*(.+)$/m,
  );
  if (cannotMatch) {
    return {
      content: "",
      isDiff: false,
      cannotFixReason: cannotMatch[1].trim(),
      model,
      tier: request.tier,
    };
  }

  const hasDiff =
    content.includes("--- a/") ||
    content.includes("+++") ||
    content.includes("@@");

  return {
    content,
    isDiff: hasDiff,
    model,
    tier: request.tier,
  };
}
