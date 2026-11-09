// Live Fast inference via curl; fixture tools are local, deterministic and safe.
import { mkdirSync, writeFileSync } from "node:fs";
import { loadCookieHeader } from "./vault.mjs";
import { curlSession } from "./curl-session.mjs";
import { sendRpc } from "./rpc-request.mjs";
import { form } from "./payload.mjs";
import { rpcResult } from "./rpc-result.mjs";
import { variants } from "./tool-probe-prompts.mjs";
import { trial } from "./tool-probe-trial.mjs";

const repetitions = Number(process.argv[2] ?? 2);
if (!Number.isInteger(repetitions) || repetitions < 1 || repetitions > 3) throw new Error("Choose 1-3 repetitions");
const directory = `artifacts/gemini-fast-tools-${new Date().toISOString().replace(/[-:.]/g, "")}`;
mkdirSync(directory, { recursive: true, mode: 0o700 });
const evidence = { evidenceLevel: "static/local curl requests to live Google; mocked local tools",
  requestedModel: "gemini-web-fast", requestedMode: "fbb127bbb056c959",
  backendIdentity: "Not attested", nativeAgentRuntimeTested: false, trials: [] };
try {
  const cookie = loadCookieHeader();
  const session = await curlSession(cookie, "--http2", () => {});
  if (!session) throw new Error("bootstrap_failed");
  evidence.authenticatedTokenMarker = session.authenticatedToken;
  const secrets = [cookie, session.at, session.sid, process.env.VAULT_TOKEN ?? "",
    ...cookie.split(/;\s*/).map(pair => pair.slice(pair.indexOf("=") + 1))];
  const ask = async prompt => rpcResult(
    await sendRpc(cookie, session, evidence.requestedMode, form(prompt, session.at)), secrets, "");
  for (let repetition = 1; repetition <= repetitions; repetition++) {
    for (const variant of variants) {
      console.log(JSON.stringify({ event: "trial_start", directory, variant: variant.name, repetition }));
      const result = { repetition, ...await trial(variant, ask) };
      evidence.trials.push(result);
      writeFileSync(`${directory}/summary.json`, `${JSON.stringify(evidence, null, 2)}\n`, { mode: 0o600 });
      console.log(JSON.stringify({ event: "trial_result", directory, variant: result.variant, repetition,
        calls: result.executions.map(item => item.call), completed: result.completed,
        failure: result.failure, finalAnswer: result.turns.at(-1)?.answer }));
    }
  }
  if (!evidence.trials.some(result => result.completed)) process.exitCode = 1;
} catch {
  console.error(JSON.stringify({ directory, error: "probe_failed_sensitive_details_suppressed" }));
  process.exitCode = 2;
}
