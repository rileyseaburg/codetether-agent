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

The source ownership and physical relocation checks above passed, but the
subsequent user-supplied compiler output exposed errors that these checks had
missed. Marking the overall goal complete was premature. Static audits do not
establish that Rust compiles or that runtime tests pass.

## Compiler-reported follow-up

The user reported E0364 for the public residency `forget` re-export and invalid
semicolon separators after macro visibility fragments. The follow-up makes
both residency cleanup functions public because application macros invoke them
across crate boundaries. It changes all 20 affected macro matchers and their
callers/examples to comma separators, and removes three reported unused
imports in plugin verification.

The declaration audit now explicitly rejects semicolons after visibility
fragments throughout the collected macro definitions. Both the single and
paired visibility failure shapes have negative probes. Its seven probes and
151 application expansions pass, as do the nine Node regression cases,
workspace/dependency audits, file-limit check and standalone formatting check.
These are static results only. Compilation and Rust tests have not been rerun;
the no-Cargo, no-rustc and no-Python restrictions remain in force.
