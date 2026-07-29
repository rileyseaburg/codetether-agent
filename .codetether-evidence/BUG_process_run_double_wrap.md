# Bug report: `process_run` exposed two `Result` layers on the plugin path

**Filed against:** CodeTether Agent (`src/tool/tetherscript/runner/`)
**Not a TetherScript bug** — see "Why this is not upstream" below.
**Found:** while building a live regression guard for the `gemini-web` tool-failure report.
**Status:** fixed in this branch.

## Symptom

The same plugin source could not be correct on both execution paths:

| path | `process_run(...)?` | `process_run(...)??` |
|---|---|---|
| `tetherscript run` | works | `? operator applied to map, expected Result` |
| `tetherscript_plugin` tool | `cannot index result with str` | works |

Reproduced with a two-hook plugin calling `process_run` identically, differing
only in unwrap arity.

## Why it mattered

This silently corrupted a live measurement. A TetherScript guard that shells out
to `codetether` and counts tool executions reported **0/10 failures**, while the
identical command run by hand succeeded **6/8**. The guard was not observing the
model at all: `process_run` returned an inner `Result` that the classifier
stringified instead of indexing, so every trial looked like a failure.

A wrong-but-plausible zero is worse than a crash. Had it not been cross-checked
by hand, it would have "confirmed" a bug that was already fixed.

## Root cause

TetherScript's authority contract (`interp.rs`, capability dispatch) states:

> Every authority method returns a TetherScript Result so callers can use `?` or
> `.is_ok()` uniformly. Native-side `Ok(v)` lifts to `Value::Result(Ok(v))`.

So the interpreter adds exactly one `Result` layer. Our authority added a second:

```rust
// src/tool/tetherscript/runner/process_authority.rs (before)
fn invoke(&self, ..) -> Result<Value, String> {
    match method {
        "run" => Ok(run(self.progress_id.as_deref(), args)),  // lifted by interp
        ..
    }
}

fn run(progress_id: Option<&str>, args: &[Value]) -> Value {
    match progress_id {
        // process_run::run already returns Result<Value, String>;
        // wrap() turned it into a *second* Value::Result layer.
        Some(id) => process_types::wrap(process_run::run(id, args)),
        None => tetherscript::system::process_run(args),
    }
}
```

Result: `Result<Result<map>>` when a `progress_id` was present (the plugin-tool
path), but `Result<map>` otherwise (the `tetherscript run` path). The two
branches disagreed on arity.

## Fix

Return a bare host `Result` from `invoke` and let the interpreter do the single
lift, per the documented contract. The `system::*` branch is unwrapped once so
both branches agree:

```rust
fn run(progress_id: Option<&str>, args: &[Value]) -> Result<Value, String> {
    match progress_id {
        Some(id) => process_run::run(id, args),
        None => process_types::unwrap_result(tetherscript::system::process_run(args)),
    }
}
```

`process_types::wrap` was renamed to `unwrap_result` with the inverse meaning,
because "wrap" was what invited the double layer.

## Why this is not upstream

TetherScript behaves correctly and consistently. The contract is documented in
its own dispatch code, and `system::process_run` honors it. Only CodeTether's
`ProcessAuthority` violated it. No upstream PR is warranted; if anything,
upstream could add a debug assertion rejecting an authority that returns an
already-wrapped `Value::Result`, which would have caught this immediately.

## Compatibility — breaking, and I initially got this wrong

My first assessment claimed `??` plugins would keep working, based on a live
plugin-runtime check. That check was invalid: the installed binary predated the
fix, so it exercised the old double-wrapped path.

Against the fixed code, `??` is a **hard error**:

    plugin failed: ? operator applied to map, expected Result

Two checked-in plugins needed migration and were updated to a single `?`:

- `examples/tetherscript/bash_guard.tether` (2 call sites)
- `examples/tetherscript/bedrock_model_probe.tether` (1 call site)

The regression surfaced in the existing `bash_guard` suite, not in my own
migration test, which had permissively accepted either outcome. That test was
rewritten to assert the break is loud and names the arity problem.

Lesson: a live check against a stale binary is not evidence.

## Tests

- `process_run_exposes_exactly_one_result_layer` — a single `?` indexes the
  result map on the plugin path.
- `single_unwrap_is_the_supported_arity` — one `?` indexes the result map.
- `stale_double_unwrap_fails_loudly_instead_of_silently` — `??` must fail with an
  error naming the arity problem, never succeed silently.

Both in `src/tool/tetherscript/runner/`.
