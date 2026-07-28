# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.1.0-alpha.16] - 2026-05-18

### Added

- Added native visual element evidence to `BrowserPage::production_debug_report()`,
  including selector candidates, computed styles, visibility, and layout bounds
  for React-style production UI debugging.
- Added React-style controlled form interaction coverage for native agent
  actions: live `value`/`checked` reads, user-like click event ordering,
  prevented submit handling, Enter-to-submit, and HAR-visible POST bodies.
- Added native fetch/XHR server-cookie propagation so `Set-Cookie` login
  responses update the session jar, keep `HttpOnly` hidden from
  `document.cookie`, and authenticate later routed requests with `Cookie`.

## [0.1.0-alpha.15] - 2026-05-17

### Added

- Added regression coverage for a React-style ESM `createRoot(...).render(...)`
  flow that mutates the native browser DOM from registered module resources.
- Added browser JavaScript regression coverage for pending `await` microtask
  resolution, Promise `.then()` adoption of handler-returned pending promises,
  and XHR `loadend`/`response`/`responseType` parity used by Axios-style
  production request adapters.
- Added a `tethercstp_browser.tether` example that mounts a React-style app
  through `ReactDOM.createRoot(...).render(...)` and renders the mounted result.
- Expanded experimental browser runtime built-ins with CSS rule introspection,
  computed styles, query selection, text extraction, page snapshots,
  framework-root/resource discovery, and structured display-list output.

### Fixed

- Fixed deterministic module resource resolution so relative imports can match
  registered absolute page paths as well as fully resolved URLs.
- Fixed browser module rewriting for default imports and `export default`
  bindings used by React-style module bundles.
- Removed the external HTTP client dependency from the capability path so the
  crate remains zero-dependency and `cargo install --path .` does not lock or
  download transitive packages.
- Split the HTTP capability authority into focused modules that satisfy the
  changed-file 50-line source ratchet.
- Added zero-Rust-dependency HTTPS support to `http_get`/`http_request` through
  the existing platform TLS shim, including Windows Git OpenSSL discovery.
- Added a live `realm_micro1_login_probe.tether` script that fetches the Realm
  login page through tetherscript and exposes the current production-bundle
  execution gap.
- Added fast diagnostics for unsupported modern JavaScript bundle syntax so
  tetherscript browser probes report the blocking construct instead of timing
  out while parsing large inlined scripts.
- Fixed native browser execution gaps found against the live Realm React login
  bundle, including route-regex `String.prototype.match()` captures,
  `Array.prototype.reduceRight()`, browser `await` unwrapping, deterministic
  pending-promise microtask draining, Promise adoption, and XHR lifecycle
  response fields. The Realm login root now mounts and renders through the
  native tetherscript browser path while the live profile request records its
  401 network event.

## [0.1.0-alpha.14] - 2026-05-14

### Added

- Added native browser-agent release coverage for external resources, classic
  scripts, module scripts, static module imports, dynamic `import()`, DOM
  assertions, traces, and CLI browser grants.
- Added `BrowserPage::production_debug_report()` with console-error, page-error,
  HAR-style network entries, source-mapped page-error stack frames,
  failed-request, source-map, framework, classified runtime exception, and React
  hydration diagnostics for bundled production UI validation.
- Added source-map resource registration for deterministic production bundles.
- Added a native-browser contract test that rejects external browser engines
  and remote-control drivers as browser backends.
- Added `LICENSE-MIT`, `CONTRIBUTING.md`, and GitHub Actions
  CI for format, clippy, tests, doc tests, rustdoc warnings, file limits, and
  package verification.
- Added `check_file_limits.sh` and `check_file_limits.ps1` to enforce the
  50-line source-file ratchet for changed `src/**/*.rs` files.
- Added VM and ownership regression tests for function-local bindings, moved
  function locals, immutable assignment rejection, and scalar Copy moves.

### Changed

- Prepared crates.io metadata for `0.1.0-alpha.14`, including the canonical
  repository URL, dual MIT/Apache-2.0 license metadata, and a strict package
  include list.
- Split newly expanded source files into focused modules so changed Rust source
  files respect the file-limit ratchet, including CSS layout, selector healing,
  source discovery, and browser support helpers.
- Updated browser capability documentation and the agent browser contract.

### Fixed

- Fixed rustdoc links so `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`
  passes cleanly.
- Fixed VM function-call binding behavior so parameters and locals use the
  environment path that preserves mutability and move tombstone semantics.
- Fixed static ownership analysis so moving Copy scalar values does not mark
  the source binding as moved.
- Prevented internal/dev artifacts such as `AGENTS.md`, editor files, Docker
  files, benchmark scripts, and `experiments/examples` from entering the crate
  package.

## [0.1.0-alpha.11] - 2026-05-11

### Added

- Added integration tests that run the core example programs and verify stdout against checked-in golden files.
- Added regression coverage for the use-after-move example's expected ownership error.

## [0.1.0-alpha.8] - 2026-05-02

### Added

- Dependency-free JavaScript interpreter (js module) with globals, functions, control flow, arrays, and js_eval.
- Browser JavaScript host bindings (browser_js module) exposing window/document, DOM mutation/querying, events, deterministic timers, and Storage APIs.
- Expanded browser subsystem with richer CSS selector parsing/cascade, structured snapshot and display-list output helpers.
- tetherscript js CLI subcommand for running JavaScript.
- Test coverage for img src attribute in display commands.

### Fixed

- EVENT_REGISTRY thread_local now cleared per evaluation to prevent memory leaks.
- <img src> DOM attribute carried into LayoutBox.styles so DisplayCommand::Image.src is non-empty.

## [0.1.0-alpha.6] - 2026-05-01

### Added

- Added first-class `Bytes` support across the language pipeline:
  - `Value::Bytes` runtime representation.
  - `b"..."` byte-string literals with `\xNN` escapes.
  - `bytes(...)` builtin for strings, byte lists, and bytes cloning.
  - Bytes indexing, index assignment, iteration, equality, truthiness, and display formatting.
  - Bytes methods: `len`, `push`, `pop`, `decode_utf8`, `to_string`, and `hex`.
- Added static ownership analysis in `src/ownership.rs`.
- Added `tetherscript check <file>` for parse plus statically-resolvable ownership checks.
- Added pre-execution ownership analysis to `tetherscript run`.
- Added VM instruction-budget enforcement for bytecode execution.
- Added `VM::builder()`, `VmBuilder`, and `tetherscript::Vm` re-export for embedders.
- Added tests covering bytes behavior in both interpreter and VM, plus bytes JSON encoding.

### Changed

- `tetherscript run <file>` now uses the bytecode VM by default.
- Added `--interp` / `--tree-walk` to run with the tree-walking interpreter for debugging.
- Normalized prerelease versioning from `0.0.1-alpha-0.5` to `0.1.0-alpha.6`.
- Completed crate metadata for publishing, including repository, readme, keywords, and categories.
- JSON encoding now represents bytes as arrays of integers without an intermediate `Vec<Value>` allocation.
- `bytes.hex()` now avoids per-byte temporary string allocations.

### Fixed

- Fixed VM byte literal semantics so mutable byte constants are deep-cloned on load and do not share buffers across evaluations.
- Fixed duplicate ownership diagnostics for borrow bindings.
- Fixed `fs.read` binary fallback to avoid cloning the entire file buffer when UTF-8 decoding fails.

## [0.1.0-alpha.5] - 2026-05-01

### Changed

- Initial alpha-stable feature publication. Superseded by `0.1.0-alpha.6` with PR review fixes.
