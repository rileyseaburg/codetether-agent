# Resumable LM Studio text jobs

`scripts/lmstudio-jobs/cli.mjs` sends text-only requests through the authenticated
CodeTether `/v1/chat/completions` API, not directly to LM Studio. Thus the deployed
provider's queue and budget remain authoritative. No model-selected tools run.

Create a JSON specification such as:

```json
{"objective":"Explain each section accurately; preserve caveats.","steps":["Section one text","Section two text"],"output":4096,"maxCalls":16,"totalOutput":32768,"seconds":3600}
```

Alternatively supply `document` instead of `steps`: it is split without losing
Unicode into at most 32 chunks of 6000 UTF-8 bytes. Objective text stays unchanged
in every call. Completed parts are retained; a separately budgeted summary passes
bounded, **lossy** notes between steps. Summaries are context aids, not a claim that
every fact fits into the next request. Check the saved parts for authoritative output.

```sh
node scripts/lmstudio-jobs/cli.mjs create job.json
# Supply CODETETHER_AUTH_TOKEN securely in the environment, never in the JSON.
node scripts/lmstudio-jobs/cli.mjs run JOB_ID
node scripts/lmstudio-jobs/cli.mjs status JOB_ID
node scripts/lmstudio-jobs/cli.mjs cancel JOB_ID
node scripts/lmstudio-jobs/cli.mjs resume JOB_ID
```

Default endpoint: `https://server.codetether.run`; override with `CODETETHER_URL`.
HTTPS is required except on loopback. Endpoint changes on resume are rejected.
Checkpoints live under `~/.local/share/codetether-lmstudio-jobs/` (override with
`CODETETHER_JOB_ROOT`). Directories/files are private; writes use fsync plus atomic
rename. Keep this on a local filesystem, not a shared NAS; locks are host-local.

Each attempt reserves output budget **before** HTTP. Confirmed usage can refund
unused tokens; missing usage and interrupted attempts cannot. Token, call and
wall-clock budgets survive resume. `length` triggers bounded continuation; exact
repeated output stops as `stalled`. `exhausted` means partial work, not completion.
An interrupted request becomes `uncertain`; only explicit `resume` may retry it.
Exactly-once inference is not promised. Cancellation aborts active HTTP and prevents
late results from advancing the job; cancelled jobs do not resume silently.

The runner prints committed-step progress, not token-level streaming; normal chat
streaming remains available separately. Tests: `node --test scripts/lmstudio-jobs/*.test.mjs`.

An `empty_answer` failure can occur when a tiny output cap is exhausted before the
model emits visible answer text. Create a separate job with a larger allowance;
the runner never silently raises the original job's declared budget.