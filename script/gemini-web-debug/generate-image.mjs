import { IMAGE_MODE } from "./image-payload.mjs";
import { imageOutput } from "./image-output.mjs";
import { ImageError } from "./image-error.mjs";
import { imageWorkflow } from "./image-workflow.mjs";

const [prompt, destination, previousSelection, ...extra] = process.argv.slice(2);
if (!prompt?.trim() || extra.length || prompt === "--help") {
  console.log('Usage: node script/gemini-web-debug/generate-image.mjs "image prompt" [new-output-directory] [previous-output-or-latest]');
  process.exitCode = prompt === "--help" ? 0 : 2;
} else {
  const directory = destination ?? `gemini-images-${new Date().toISOString().replace(/[-:.]/g, "")}`;
  /** @type {ReturnType<typeof imageOutput> | undefined} */ let save;
  /** @type {import("./image-workflow.mjs").WorkflowState} */ const state = {stage: "output", images: []};
  const evidence = { evidenceLevel: "static/local client diagnostic against live Google HTTP",
    runtime: "Node RPC image client with curl HTTP/2", requestedSelector: IMAGE_MODE, backendIdentity: "Not attested",
    credentialSource: process.env.GEMINI_WEB_COOKIE_FILE ? "explicit Netscape cookie file" : "Vault" };
  try {
    save = imageOutput(directory);
    console.log(JSON.stringify({ event: "image_run", directory, ...evidence }));
    const result = await imageWorkflow({prompt, directory, previousSelection}, state,
      (label, value) => save?.(label, {...evidence, ...value}));
    const summary = { ...evidence, outcome: "ok", images: state.images, ...result };
    save("summary", summary); console.log(JSON.stringify({ directory, ...summary }));
  } catch (error) {
    const summary = { ...evidence, outcome: "error", ...state,
      code: error instanceof ImageError ? error.message : "client_operation_failed",
      details: error instanceof ImageError ? error.details : {} };
    try { save?.("failure", summary); } catch { /* Preserve partial files even when evidence writes fail. */ }
    console.error(JSON.stringify({ directory, ...summary })); process.exitCode = 2;
  }
}