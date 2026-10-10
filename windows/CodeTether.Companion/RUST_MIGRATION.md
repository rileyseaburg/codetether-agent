# Rust migration: protocol and session foundations

The Windows companion is a Rust project. The C#/.NET source is a behavior
reference only: do not build, run, test, package, or deploy it. Missing .NET
tooling is not a blocker. The relay reference is Node.js/TypeScript, and the
phone remains native Swift/SwiftUI. The first migration slice adds
`crates/codetether-companion-protocol` at the repository root; this library
does **not** provide or launch a Rust Windows executable.

## Cargo workspace layout: one Windows application

The companion is one application composed of crates in the existing
repository-root Cargo workspace. It does not introduce a nested workspace or
an additional companion process:

- `windows/CodeTether.Companion/rust-shell`: package
  `codetether-companion-shell`, owning the sole `codetether-companion` binary.
- `windows/CodeTether.Companion/desktop-staging`: internal library package
  `codetether-companion-desktop` for native desktop and monitor boundaries.
- `crates/codetether-companion-{protocol,core,capture}`: shared library crates,
  not independent applications.

Both Windows packages are listed in the root `Cargo.toml`; dependency entries
use the existing root `Cargo.lock`. The shell implementation is still in
progress, including native controls, capture and pairing integration. Workspace
registration does not establish that the Windows executable builds or runs.
**Verification for this workspace change: not-run** (no builds, static checks,
tests or application runs; integration testing remains user-owned).

## Hosted inference and workspace configuration

The companion uses the existing hosted CodeTether service at
`https://server.codetether.run/v1/chat/completions` through the authenticated
relay. It must not start, embed or supervise a local CodeTether agent/server.
The relay keeps owner inference credentials server-side; Windows receives
only the limited, memory-only device capability. Screen analysis is a direct,
streamed completion with `tools: []`, not a tool-capable agent turn; the
companion is not read-only — the agent must be able to type through the
Windows app into chat boxes, and that capability must be implemented and
documented as an explicit wire and native-input contract.

`workspace.Cargo.toml` is a reference-only companion subset of the root
manifest, including the protocol, core, capture-policy, desktop adapter and
sole Windows binary crates. The former nested `Cargo.toml` is removed; do not
rename the reference file to `Cargo.toml`. Cargo operations use
`../../Cargo.toml` and `../../Cargo.lock`, without a root-agent dependency.

## Native selected-monitor capture (COMP-004)

`desktop-staging` now exposes `capture_selected(&Monitor, &AtomicBool)` and
`CapturedFrame`. It captures only the selected monitor's physical rectangle,
rechecks eligibility/selection around native work, and supports cooperative
per-operation cancellation. Native resources are RAII-owned; the DIB borrows
its thread-bound device context. No disk or network operation is performed.

Source geometry is limited to 34 million pixels. Bilinear resize never
upscales and caps the longest edge at 1600 pixels. JPEG quality attempts
75/55/35/20 use a hard 524288-byte output limit. The encoder dependency is
`jpeg-encoder` 0.6.1, avoiding the Rust 1.88 requirement of the previously
selected `image` release while retaining the crate's Rust 1.85 floor.

Adapter-owned DIB/RGB/JPEG buffers are cleared on drop, not OS/driver or
third-party encoder scratch buffers. Consent and stale-generation checks
remain host responsibilities; see `desktop-staging/README.md`. The shell now
authorizes background owner requests through local pairing and monitor selection;
it does not require a separate Windows Start action.
Dependency resolution is not compilation evidence.

**Verification: not-run** (no builds, static checks, tests or live requests).

## Authenticated Windows device transport (COMP-005)

`rust-shell/src/relay/` implements the device side of the existing relay
contract. Pairing validates the full receipt (session UUID, 256-bit URL-safe
device token, 15–300-second interval and one-hour-bounded future expiry).
Authenticated polling returns only the opaque owner-request UUID; the owner's
question is never sent to Windows. Upload accepts an already bounded
`CapturedFrame`, validates its dimensions and request ID, base64-encodes it in
zeroizing memory and requires the typed HTTP 202 acknowledgement.

All requests use the fixed `https://server.codetether.run/companion` origin,
reject redirects, apply 10-second connect and 20-second total timeouts, send
`Cache-Control: no-store`, and bound response bodies to 8 KiB. Device tokens,
serialized request bodies and response bodies are memory-only zeroizing
buffers. Errors do not include credentials, pairing codes or screenshot data;
401, 403, 404 and 410 revoke the device capability. Local Stop/unpair and Exit
clear local ownership and cancel work without waiting for network confirmation;
the last worker-owned capability is released after its bounded cleanup.

## Background iPhone requests (COMP-006)

User scope correction supersedes the old Start gate: pairing plus locally chosen
monitor and visible sharing notice arm background Capture now / Ask requests.
One worker polls every two seconds, captures off the UI thread only for an opaque
request ID, re-polls to check that ID is still current, then uploads a fresh frame.
Transient failures back off five seconds; screenshots are not queued or retained.
Periodic/right-click/double-click scheduling remains pending and disabled.

Pause/Stop/unpair/exit and desktop changes cancel the worker's async HTTP and
cooperative native capture. Each worker has a fresh capture cancellation flag;
a replacement waits for the old worker, native cleanup and bounded best-effort
relay pause notification to finish. Lock/disconnect suspend automatically; an
unchanged selected monitor and live session resume on eligibility restoration.
Explicit Pause remains latched until Resume requests. Monitor geometry changes
require selecting the monitor again. Tray tooltip/title show sharing ON or OFF.
The controls are a `WS_EX_TOOLWINDOW`, so Windows does not give them a taskbar
button. After pairing, **Run in background** hides the window without changing
pairing or sharing. Minimize and Close also hide. The notification-area tray
retains Open controls, Pause, Stop/unpair and Exit. A static/local Windows build
includes these controls; interactive behavior remains **not-run**.
Pairing results are owned by join handles and discarded after Stop or replacement;
no raw result pointer can restore an invalidated pairing. No credentials persist.
Cancellation cannot retract bytes already accepted by the relay. Remote pause
confirmation is best-effort; local suspension does not wait for that confirmation.

**Verification: not-run** (no build, test, static check, live request or Windows
application run). Offline lockfile resolution changed no package versions and
is configuration evidence only.

## Implemented boundary

### Windows delivery update — 2026-10-09

Static/local: the root-workspace `codetether-companion` build succeeded on
RILEYS-LAPTOP over `riley@192.168.50.187`, Rust/Cargo 1.98.0 and MSVC. Its
hash-matched, unsigned development executable is installed at
`C:\Users\riley\AppData\Local\Programs\CodeTetherCompanion\codetether-companion.exe`.
Open **CodeTether Screen Companion** from the Desktop or Start menu, pair,
select the monitor, then choose **Run in background**. No Windows Start step
is required. The tray retains local control when the window is hidden.
Build/installation records and exact source/archive hashes are retained in
`artifacts/windows-build-20261009T034712Z/`. The prior Windows build directory
and all earlier evidence are preserved. No tests or application launch ran.
Runtime capture/UI behavior, native typing, and release signing/packaging
are not established by this build; their respective TODO items remain open.

### Windows keyboard-delivery update — 2026-10-09

Static/local: subsequent build attempt 02 and installation exited 0 on
`riley@192.168.50.187`. This replaces the development executable above with
native Unicode `SendInput` delivery of the existing `/commands` reply queue,
acknowledged through `/typed`. Windows generates no text and sends no Enter key.
UIA verifies the focused editable target; a nonactivating preview shows target
and text. See [TYPING.md](TYPING.md) for cancellation, deduplication, single-line
limits, unsupported targets and the coarse consumption-only acknowledgement.
Evidence: `artifacts/windows-typing-20261009T040502Z/`. The installed SHA-256 is
`dc9c80fd6f19febc031d7a06b785f9bc1834551531fdb248362c5e3beedbd0e6`.
The previous executable is backed up. Tests, application launch, keyboard/UI
and live relay verification: **not-run**. The executable remains unsigned.

### Protocol foundation

The Rust library contains serde JSON models for session setup and receipts,
pairing and device capabilities, fresh-frame requests and device commands,
capture payloads/triggers, acknowledgements/errors, and the six SSE event kinds.
Small modules separate each wire concern. Sensitive payload types intentionally
do not implement `Debug`; callers must still avoid logging serialized data.

`fixtures/wire.json` in that crate is the shared compatibility source. Rust
integration tests round-trip the contracts. The existing relay's
`tests/wire-*.test.ts` consumes the same fixture, exercises its input validators,
and compares real local HTTP/SSE responses against it with mocked inference.
Only generated identifiers, event sequences, and timestamps are normalized.
The JPEG is synthetic header-only test data, not a photograph or live inference
input. All credentials in the fixture are test-only placeholders.

Important compatibility details:

- Idle device commands contain `request_id: null`; missing IDs are rejected.
- Legacy captures may omit trigger/request ID. Explicit nulls are rejected.
- Event kinds and capture triggers are closed enums; status strings remain open.
- Timestamps remain verbatim strings; this crate does not parse their semantics.
- Unknown JSON fields remain tolerated for additive protocol changes.
- Owner questions never appear in device commands.

## Verification from the repository root

```sh
cargo fmt -p codetether-companion-protocol -- --check
cargo test -p codetether-companion-protocol --offline --locked
cargo clippy -p codetether-companion-protocol --all-targets --offline --locked -- -D warnings
./check_file_limits.sh
cd scripts/public-server/companion && npm test && npm run typecheck
```

## Remaining migration work

This library enforces wire shape, not authorization, consent, JPEG bounds,
freshness, quotas, or interval policy. Keep the existing runtime checks intact.
Rust relay HTTP/SSE/inference, periodic/click scheduling and release packaging
remain pending. Windows capture and keyboard reply delivery are connected in
source; [TYPING.md](TYPING.md) documents actual SendInput delivery and its limits.
This is not end-to-end evidence. The session/authentication library below is
not a standalone companion. iOS remains Swift/SwiftUI.
Preserve .NET source for reference, never as a runtime or rollback target. No
routes, configuration, credential storage, or retention changed in this slice.

## Local verification evidence

The commands above were run for this slice:

| Check | Evidence level | Observed result |
| --- | --- | --- |
| Rust formatting | static/local | Exit 0 |
| Rust focused tests | focused CI-like | 9 integration tests and 16 doctests; exit 0 |
| Crate Clippy, all targets, warnings denied | focused CI-like | Exit 0 |
| Repository line-budget ratchet | static/local | Exit 0; checks root `src/**/*.rs` only |
| Relay tests | mocked local | 16 tests, 0 failures; exit 0 |
| Relay TypeScript checking | static/local | `tsc --noEmit`, exit 0 |

Static/local: all 11 new Rust source/test files contain 9–37 nonblank,
noncomment lines; all four new TypeScript test/helper files are under 50 lines.
Cargo reported existing workspace warnings for `toml` version metadata and
unused `candle-kernels`/`tetherscript` patches. No broad Cargo build/check was run.
New crate source/test file budgets are checked separately because the repository
script does not cover crate paths. Windows execution, iOS simulator/device,
live provider/deployment, signing, installation, and platform upload are
**not-run** for this slice.

## Second slice: memory-only authentication and session lifecycle

`crates/codetether-companion-core` adds separate owner/device authentication,
exact-origin checks, session creation, pairing, expiry, sweeping and revocation.
It uses the protocol crate's receipts without replacing the TypeScript relay.
Owner credentials and device tokens are SHA-256 hashed and compared in constant
time. Random device tokens contain 256 bits; the raw token is returned once.
Sensitive state does not implement `Debug`. Stopping discards instructions and
capabilities; restarting the process discards the entire registry.

The core preserves four-session capacity, five-minute single-use pairing codes,
one-hour sessions and a process-wide 30-attempt/minute pairing budget. Input
checks preserve ECMAScript whitespace and UTF-16 prompt-length semantics,
15–300-second capture intervals, and the relay's provider/model validation.
Shared `fixtures/security.json` cases exercise authentication, origins and code
normalization in Rust and the existing relay's security-contract tests.
Shared `fixtures/lifecycle.json` cases exercise both sides of the inclusive
five-minute pairing and one-hour session deadlines, four-session capacity and
recovery, and the exact one-minute pairing-window reset. Successful pairing
consumes the same attempt budget as a valid but incorrect code. Rust lifecycle
tests are split into expiry and budget modules to keep each file under 50 lines.

Callers must keep one registry per process behind serialized mutation, enforce
owner authorization and origin policy before owner operations, and supply a
trusted server clock in Unix milliseconds. Device pairing grants neither owner
access nor local Windows capture consent. This library contains no HTTP/SSE
transport, capture engine, inference, retention store or executable. Those
runtime migration slices remain pending. Existing relay capture checks remain
authoritative; no runtime cutover or routes changed.

Focused verification commands from the repository root:

```sh
cargo fmt -p codetether-companion-core -p codetether-companion-protocol -- --check
cargo test -p codetether-companion-core -p codetether-companion-protocol --offline --locked
cargo clippy -p codetether-companion-core -p codetether-companion-protocol --all-targets --offline --locked -- -D warnings
cd scripts/public-server/companion && npm test && npm run typecheck
```

### Native Windows library checks

`Verify-CompanionLibraries.ps1` extracts an archive containing the repository
lockfile and both crates into an isolated temporary workspace. It preserves the
original lockfile as `Cargo.workspace.lock`, permits pruning for the reduced
workspace, then runs tests and locked Clippy with warnings denied. Both lockfiles,
archive hashes, `windows-tests.log` and `windows-clippy.log` remain available.
This is **focused CI-like** native Windows library verification, not an app test.
It never starts the GUI, requests capture permission or captures screenshots.

### Second-slice verification evidence

| Check | Evidence level | Observed result |
| --- | --- | --- |
| Linux tests, both crates | focused CI-like | 23 integration tests and 22 doctests; exit 0 |
| Linux all-target Clippy, warnings denied | focused CI-like | Exit 0 |
| Native Windows tests, both crates | focused CI-like | 23 integration tests and 22 doctests; exit 0 |
| Native Windows all-target Clippy, warnings denied | focused CI-like | Exit 0 |
| Relay suite | mocked local | 22 tests, 0 failures; exit 0 |
| Formatting, TypeScript checking, whitespace | static/local | Exit 0 |
| File budgets | static/local | 36 Rust files within 50 code lines; root ratchet exit 0 |

The native Windows run used `riley@192.168.50.187` and retained its evidence in
`C:\Users\riley\AppData\Local\Temp\codetether-companion-tests-0fe859e3-b573-4a0b-8963-d944ea54fe97`.
Local copies are under `target/companion-verification/windows-lifecycle-evidence/`:
`windows-tests.log`, `windows-clippy.log`, `Cargo.workspace.lock`, and `Cargo.lock`.
The source archive remains at
`target/companion-verification/windows-source-lifecycle.tar.gz`, with SHA-256
`307d431ba04bc8f5c9d6c086c90e48cc18c67969c16333137f9f50c7bda98f39`.
Static/local archive comparison found no differences against the checked source.
The original workspace lockfile SHA-256 is
`aef6e1335aa16ff8e7fe4a864d3567464b635dfb288254856b819983e2a0d39e`;
the isolated Windows lockfile SHA-256 is
`70c6ae741471438c2140b9b118324db95eef7646ea70f9f769e4c5be35fa58c2`.
These library checks do not require or exercise a .NET runtime. No broad local
Cargo build/check was run as part of these focused checks.

**Not-run:** Rust companion GUI/capture tests; the native Rust application is
not implemented by these libraries. .NET reference execution is out of scope,
not blocked by SDK availability. Library tests do not establish native Rust UI
or capture behavior, packaging, live integration, iOS behavior, or installation.

## Third slice: deterministic capture scheduling

`crates/codetether-companion-capture` implements memory-only scheduling policy.
It is not an OS capture adapter or executable. `Schedule` starts paused; only
explicit local Start/resume may arm it after live pairing and monitor selection.
Pairing, clicks and remote requests cannot arm it. Local options enforce the
15–300-second interval and select enabled triggers without remote overrides.

Pending work is bounded to one coalesced click and one opaque request ID.
Selection prioritizes remote manual requests, then clicks, then periodic work.
Successful uploads reset the interval and five-second click cooldown; remote
requests bypass that cooldown. Failures preserve pending work and impose a
five-second retry delay on every trigger. Pause discards all pending work.

The host must serialize polling, capture, upload and completion callbacks.
`due` does not consume work or prevent concurrent host tasks. Before resuming,
the host must cancel and await old work; on lock, disconnect, unpair or stop it
must pause and independently cancel I/O. Monitor filtering, desktop eligibility,
credential revocation and authenticated request freshness remain host/relay
responsibilities. Request IDs must accompany only manual uploads.

Focused commands from the repository root:

```sh
cargo fmt -p codetether-companion-capture -- --check
cargo test -p codetether-companion-capture --offline --locked
cargo clippy -p codetether-companion-capture --all-targets --offline --locked -- -D warnings
```

### Native Windows scheduler evidence

**Focused CI-like (native Windows libraries):** the retained test log records
57 passing tests/doctests and zero failures across the three Rust libraries;
the warnings-denied Clippy log records a finished run. Evidence is retained at
`target/companion-verification/windows-scheduler-evidence/` (repository root),
including both logs and the original and reduced-workspace lockfiles.
The source archive is `target/companion-verification/windows-source-scheduler.tar.gz`;
its SHA-256 is `6f28dd93408250abb93cf7bf21c044ea0ca6b8f3b4460cc223d86dc4559af733`.

**Not-run for this slice:** OS screen capture, desktop hooks, tray/GUI behavior,
typed-input delivery, live relay/provider integration, installer/signing and iOS
device testing.

### Combined library verification

Run from the repository root; use a dedicated target directory to avoid
contending with unrelated workspace builds:

```sh
export CARGO_TARGET_DIR=target/companion-verification/linux
cargo fmt -p codetether-companion-protocol -p codetether-companion-core -p codetether-companion-capture -- --check
cargo test -p codetether-companion-protocol -p codetether-companion-core -p codetether-companion-capture --offline --locked
cargo clippy -p codetether-companion-protocol -p codetether-companion-core -p codetether-companion-capture --all-targets --offline --locked -- -D warnings
(cd scripts/public-server/companion && npm test && npm run typecheck)
```

Classify Rust library tests as **focused CI-like**, relay fixture/HTTP tests as
**mocked local**, and formatting/typechecking as **static/local**. None of these
checks supplies evidence of a runnable Windows companion or runtime cutover.