# Source consolidation completion audit

The former root application now lives in `crates/codetether-agent`. The root
manifest is a virtual workspace. Its application member contains native
composition and integration tests; extracted domain implementations, state and
policies retain their dedicated component crates.

## Requirement evidence

| Requirement | Current evidence |
| --- | --- |
| Move root `src/` implementation into crates | Root `src/` is absent. Application source is `crates/codetether-agent/src`. The relocation accounted for 1,281 source/test files and three Rust examples, with hashes and explicit path-fix exceptions. Subsequent orphan cleanup and test migration are committed separately. |
| Cohesive module and crate ownership | `workspace.json` records every package owner: 618 packages under `crates/` and the two existing Windows members. Native application composition remains separate from the extracted domain and support packages. |
| Valid workspace and dependency boundaries | The layout audit checks all 620 registered members, 2,791 production/build path edges and 62 development path edges. Package identities and dependency targets resolve. Production/build dependency traversal has no cycles. |
| Reachable application modules | The module audit reports 724 roots and no missing paths. The union of application library and binary targets leaves no application Rust source unreferenced. |
| Preserve behavior and compatibility | Relocation preserved source hashes except reviewed path/import fixes. Package identity, binary name, feature definitions and dependency versions were retained. Release, install, Docker and fixture paths were updated. Lost environment precedence and MiniMax pricing cases were restored from historical implementation evidence. Preexisting unused compatibility marker/fallback APIs were retained. |
| Keep tests with their owners | Pure component regressions moved into their owning crates. Nineteen native integration cases were re-registered without changing their bodies. Six stale utility cases were mapped to current APIs; standalone test-source links resolve. |
| Preserve unrelated working changes | Cargo.lock and the two existing A2A edits retain their original content hashes. The A2A edits remain unstaged at their relocated paths. |
| Commit authorized changes | The migration, component extractions, test registration, compatibility fixes and final audit are recorded in Git. |
| Do not run Cargo, rustc or Python | Verification used Node, Bash, Git, native file operations and standalone rustfmt. Rust tests, documentation tests, compilation, lockfile resolution and Docker builds were not executed. |

## Executed verification

- Nine Node tests pass for current/relocated package discovery, ambiguous or
  missing package rejection, file-size baselines across moves, oversized
  copies/new files, dependency aliases, cycle rejection, identity mismatch and
  missing test-source rejection.
- The shell version-bump regression passes for the legacy root package and the
  virtual-workspace layout without invoking a build tool.
- The declaration audit passes its five negative probes and expands the current
  application declaration macros with no unbound variables or duplicate
  inherent methods within its supported grammar.
- Production dependency, module-path, file-size, standalone formatting and Git
  whitespace checks pass within their documented static scopes.

Evidence is retained under `artifacts/consolidation-final-audit/`, with earlier
move comparisons and failed intermediate checks retained in their corresponding
consolidation evidence directories.

## Verification limit

This completes the requested source ownership and physical consolidation using
the permitted checks. Static audits do not establish that Rust compiles or that
runtime tests pass. Those results remain unverified under the explicit tooling
restriction; this report does not claim otherwise.
