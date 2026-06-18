export interface Env {
  OPENCODE_GO_KEY: string;
  GITHUB_APP_ID: string;
  GITHUB_APP_PRIVATE_KEY: string;
  GITHUB_APP_INSTALLATION_ID: string;
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    if (request.method !== "POST") {
      return new Response("Only POST method allowed", { status: 405 });
    }

    try {
      const payload: any = await request.json();
      const { repository, pr_number, head_sha, diff_url } = payload;

      if (!repository || !pr_number || !head_sha || !diff_url) {
        return new Response("Missing required payload fields", { status: 400 });
      }

      console.log(`Processing PR #${pr_number} on ${repository}`);

      // 1. Authenticate with GitHub App and get installation token
      const jwt = await generateJWT(env.GITHUB_APP_ID, env.GITHUB_APP_PRIVATE_KEY);
      const token = await getInstallationToken(jwt, env.GITHUB_APP_INSTALLATION_ID);

      // 2. Fetch the PR diff
      const diff = await getPRDiff(token, repository, pr_number);

      // 3. Fetch the agent definition and external reviewer prompt from master
      const agentDef = await fetchRepoFile(token, repository, ".opencode/agents/spiral-reviewer-external.md");
      const promptDef = await fetchRepoFile(token, repository, ".opencode/prompts/review-external.md");

      // 4. Call OpenCode Go
      const systemPrompt = `${agentDef}\n\n${promptDef}`;
      const userPrompt = `Please review the following PR diff:\n\n\`\`\`diff\n${diff}\n\`\`\``;

      const aiResponse = await callOpenCode(env.OPENCODE_GO_KEY, systemPrompt, userPrompt);
      const aiText = aiResponse.choices[0].message.content;

      // 5. Determine Check Run conclusion
      let conclusion: "success" | "failure" | "neutral" = "success";
      if (aiText.includes("🔴")) {
        conclusion = "failure";
      } else if (aiText.includes("🟡")) {
        conclusion = "neutral";
      }

      // 6. Post the comment to the PR
      await postPRComment(token, repository, pr_number, aiText);

      // 7. Create/Update Check Run
      await createCheckRun(token, repository, head_sha, conclusion, aiText);

      return new Response(JSON.stringify({ success: true, conclusion }), {
        headers: { "Content-Type": "application/json" }
      });
    } catch (err: any) {
      console.error(err);
      return new Response(JSON.stringify({ error: err.message }), {
        status: 500,
        headers: { "Content-Type": "application/json" }
      });
    }
  }
};

// -- Helpers --

async function generateJWT(appId: string, pemKey: string): Promise<string> {
  const cleanPem = pemKey
    .replace(/-----BEGIN RSA PRIVATE KEY-----/, "")
    .replace(/-----END RSA PRIVATE KEY-----/, "")
    .replace(/-----BEGIN PRIVATE KEY-----/, "")
    .replace(/-----END PRIVATE KEY-----/, "")
    .replace(/\s+/g, "");

  const binaryKey = Uint8Array.from(atob(cleanPem), c => c.charCodeAt(0));

  const cryptoKey = await crypto.subtle.importKey(
    "pkcs8",
    binaryKey,
    {
      name: "RSASSA-PKCS1-v1_5",
      hash: "SHA-256",
    },
    false,
    ["sign"]
  );

  const now = Math.floor(Date.now() / 1000);
  const header = { alg: "RS256", typ: "JWT" };
  const payload = {
    iat: now - 60,      // clock skew tolerance
    exp: now + 540,     // 9 minutes
    iss: appId,
  };

  const textEncoder = new TextEncoder();
  const headerBase64 = b64url(textEncoder.encode(JSON.stringify(header)));
  const payloadBase64 = b64url(textEncoder.encode(JSON.stringify(payload)));

  const dataToSign = textEncoder.encode(`${headerBase64}.${payloadBase64}`);
  const signature = await crypto.subtle.sign(
    "RSASSA-PKCS1-v1_5",
    cryptoKey,
    dataToSign
  );

  const signatureBase64 = b64url(new Uint8Array(signature));
  return `${headerBase64}.${payloadBase64}.${signatureBase64}`;
}

function b64url(bytes: Uint8Array): string {
  return btoa(String.fromCharCode(...bytes))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=/g, "");
}

async function getInstallationToken(jwt: string, installationId: string): Promise<string> {
  const res = await fetch(`https://api.github.com/app/installations/${installationId}/access_tokens`, {
    method: "POST",
    headers: {
      "Authorization": `Bearer ${jwt}`,
      "Accept": "application/vnd.github+json",
      "User-Agent": "spiral-fork-reviewer"
    }
  });
  if (!res.ok) {
    throw new Error(`Failed to get installation token: ${await res.text()}`);
  }
  const data: any = await res.json();
  return data.token;
}

async function getPRDiff(token: string, repository: string, prNumber: number): Promise<string> {
  const res = await fetch(`https://api.github.com/repos/${repository}/pulls/${prNumber}`, {
    headers: {
      "Authorization": `token ${token}`,
      "Accept": "application/vnd.github.v3.diff",
      "User-Agent": "spiral-fork-reviewer"
    }
  });
  if (!res.ok) {
    throw new Error(`Failed to fetch diff: ${await res.text()}`);
  }
  return res.text();
}

async function fetchRepoFile(token: string, repository: string, path: string): Promise<string> {
  const res = await fetch(`https://api.github.com/repos/${repository}/contents/${path}?ref=master`, {
    headers: {
      "Authorization": `token ${token}`,
      "Accept": "application/vnd.github.v3.raw",
      "User-Agent": "spiral-fork-reviewer"
    }
  });
  if (!res.ok) {
    // Fall back to main branch
    const res2 = await fetch(`https://api.github.com/repos/${repository}/contents/${path}?ref=main`, {
      headers: {
        "Authorization": `token ${token}`,
        "Accept": "application/vnd.github.v3.raw",
        "User-Agent": "spiral-fork-reviewer"
      }
    });
    if (!res2.ok) {
      throw new Error(`Failed to fetch file ${path}: ${await res2.text()}`);
    }
    return res2.text();
  }
  return res.text();
}

async function callOpenCode(apiKey: string, systemPrompt: string, userPrompt: string): Promise<any> {
  const res = await fetch("https://opencode.ai/zen/go/v1/chat/completions", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${apiKey}`
    },
    body: JSON.stringify({
      model: "opencode-go/minimax-m3",
      messages: [
        { role: "system", content: systemPrompt },
        { role: "user", content: userPrompt }
      ],
      temperature: 0.1
    })
  });
  if (!res.ok) {
    throw new Error(`OpenCode Go API error: ${await res.text()}`);
  }
  return res.json();
}

async function postPRComment(token: string, repository: string, prNumber: number, body: string): Promise<void> {
  const res = await fetch(`https://api.github.com/repos/${repository}/issues/${prNumber}/comments`, {
    method: "POST",
    headers: {
      "Authorization": `token ${token}`,
      "Content-Type": "application/json",
      "Accept": "application/vnd.github+json",
      "User-Agent": "spiral-fork-reviewer"
    },
    body: JSON.stringify({ body })
  });
  if (!res.ok) {
    throw new Error(`Failed to post comment: ${await res.text()}`);
  }
}

async function createCheckRun(
  token: string,
  repository: string,
  headSha: string,
  conclusion: "success" | "failure" | "neutral",
  summary: string
): Promise<void> {
  const res = await fetch(`https://api.github.com/repos/${repository}/check-runs`, {
    method: "POST",
    headers: {
      "Authorization": `token ${token}`,
      "Content-Type": "application/json",
      "Accept": "application/vnd.github+json",
      "User-Agent": "spiral-fork-reviewer"
    },
    body: JSON.stringify({
      name: "opencode / review",
      head_sha: headSha,
      status: "completed",
      conclusion,
      output: {
        title: `Automated Review (${conclusion})`,
        summary
      }
    })
  });
  if (!res.ok) {
    throw new Error(`Failed to create Check Run: ${await res.text()}`);
  }
}
