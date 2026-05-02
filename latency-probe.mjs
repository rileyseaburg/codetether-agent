#!/usr/bin/env node
// node latency-probe.mjs --model gpt-5.5-fast --rounds 3
// Measures raw connectivity + TTFT + TTL against the OpenAI Responses API.
// Works with either OPENAI_API_KEY or the ChatGPT OAuth access token
// from Vault (via OPENAI_ACCESS_TOKEN + OPENAI_ACCOUNT_ID).

const model = process.argv.includes("--model")
  ? process.argv[process.argv.indexOf("--model") + 1]
  : "gpt-5.5-fast";
const rounds = process.argv.includes("--rounds")
  ? Number(process.argv[process.argv.indexOf("--rounds") + 1])
  : 3;

const apiKey = process.env.OPENAI_API_KEY;
const accessToken = process.env.OPENAI_ACCESS_TOKEN;
const accountId = process.env.OPENAI_ACCOUNT_ID || "";

const bearer = apiKey || accessToken;
if (!bearer) {
  console.error("ERROR: OPENAI_API_KEY or OPENAI_ACCESS_TOKEN env var required");
  process.exit(1);
}

// ChatGPT OAuth uses a different base URL than API keys
const isChatgpt = !apiKey && accessToken;
const CHATGPT_URL = "https://chatgpt.com/backend-api/codex";
const OPENAI_URL = "https://api.openai.com/v1/responses";
const URL = isChatgpt ? CHATGPT_URL : OPENAI_URL;

function authHeaders() {
  const h = { Authorization: `Bearer ${bearer}`, "Content-Type": "application/json" };
  if (accountId) {
    h["chatgpt-account-id"] = accountId;
  }
  if (isChatgpt) {
    h["User-Agent"] = "codetether-responses-ws/1.0";
    h["oai-dm-tls-collector"] = "1";
    h["origin"] = "https://chatgpt.com";
    h["referer"] = "https://chatgpt.com/";
  }
  return h;
}

async function pingOnce(round) {
  const body = {
    model,
    input: "Say 'pong' and nothing else.",
    stream: true,
    store: false,
  };

  const dnsStart = performance.now();
  let firstTokenAt = null;
  let lastTokenAt = null;
  let chunks = 0;
  let status = null;

  const ctrl = new AbortController();
  const timeout = setTimeout(() => ctrl.abort(), 60_000);

  try {
    const resp = await fetch(URL, {
      method: "POST",
      headers: authHeaders(),
      body: JSON.stringify(body),
      signal: ctrl.signal,
    });

    const connectMs = performance.now() - dnsStart;
    status = resp.status;

    if (!resp.ok) {
      const text = await resp.text();
      clearTimeout(timeout);
      return { round, status, connectMs, error: text.slice(0, 200) };
    }

    const reader = resp.body.getReader();
    const decoder = new TextDecoder();

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      chunks++;
      if (!firstTokenAt) firstTokenAt = performance.now();
      lastTokenAt = performance.now();
      // consume the bytes
      decoder.decode(value, { stream: true });
    }

    clearTimeout(timeout);
  } catch (err) {
    clearTimeout(timeout);
    return {
      round,
      status: status ?? "ERR",
      connectMs: performance.now() - dnsStart,
      error: err.message,
    };
  }

  const ttft = firstTokenAt ? firstTokenAt - dnsStart : null;
  const ttl = lastTokenAt ? lastTokenAt - dnsStart : null;

  return {
    round,
    status,
    connectMs: Math.round(connectMs),
    ttftMs: ttft ? Math.round(ttft) : null,
    ttlMs: ttl ? Math.round(ttl) : null,
    chunks,
  };
}

function ms(v) {
  return v != null ? `${v}ms` : "—";
}

async function main() {
  console.log(`OpenAI Latency Probe`);
  console.log(`  Model:   ${model}`);
  console.log(`  Endpoint: ${URL}`);
  console.log(`  Rounds:  ${rounds}`);
  console.log();

  const results = [];
  for (let i = 1; i <= rounds; i++) {
    process.stdout.write(`  Round ${i}/${rounds}...`);
    const r = await pingOnce(i);
    results.push(r);
    if (r.error) {
      console.log(` FAIL (${r.status}): ${r.error}`);
    } else {
      console.log(
        ` connect=${ms(r.connectMs)} ttft=${ms(r.ttftMs)} ttl=${ms(r.ttlMs)} chunks=${r.chunks}`
      );
    }
  }

  const ok = results.filter((r) => !r.error);
  if (ok.length > 0) {
    const avg = (arr) => Math.round(arr.reduce((a, b) => a + b, 0) / arr.length);
    const connects = ok.map((r) => r.connectMs);
    const ttfts = ok.map((r) => r.ttftMs).filter(Boolean);
    const ttls = ok.map((r) => r.ttlMs).filter(Boolean);

    console.log();
    console.log(`  ── Summary (${ok.length}/${rounds} ok) ──`);
    console.log(`  Connect:  avg=${ms(avg(connects))}  min=${ms(Math.min(...connects))}  max=${ms(Math.max(...connects))}`);
    if (ttfts.length) {
      console.log(`  TTFT:     avg=${ms(avg(ttfts))}  min=${ms(Math.min(...ttfts))}  max=${ms(Math.max(...ttfts))}`);
    }
    if (ttls.length) {
      console.log(`  TTL:      avg=${ms(avg(ttls))}  min=${ms(Math.min(...ttls))}  max=${ms(Math.max(...ttls))}`);
    }
  }

  // DNS / connectivity baseline via /models endpoint (lightweight)
  console.log();
  process.stdout.write("  Baseline GET /v1/models...");
  try {
    const t0 = performance.now();
    const resp = await fetch("https://api.openai.com/v1/models", {
      headers: { Authorization: `Bearer ${bearer}` },
      signal: AbortSignal.timeout(10_000),
    });
    const dt = Math.round(performance.now() - t0);
    console.log(` ${resp.status} in ${dt}ms`);
  } catch (e) {
    console.log(` ERR: ${e.message}`);
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
