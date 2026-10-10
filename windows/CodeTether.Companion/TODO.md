# Rust Windows Companion — Delivery Tickets

_Status reviewed from local source plus a Windows build and launch smoke on
RILEYS-LAPTOP (`ssh riley@192.168.50.135`, static IP). Evidence levels are cited per
item; anything without a cited level is **not-run**._

## Working agreement

- Build features, not more test suites. User owns app-level integration testing.
- Keep existing tests and evidence; do not add or run tests for this work.
- Rust is the companion runtime; C#/.NET stays reference-only.
- Deliver one Windows application in the existing root Cargo workspace:
  `rust-shell` owns the binary; `desktop-staging` is an internal library.
- Use hosted inference at `https://server.codetether.run`; do not start or
  embed a local CodeTether agent/server. Owner inference credentials remain
  on the relay, not in the Windows application.
- `Done` means the ticket's implementation is present, not that the user has
  verified the app end to end. Record verification separately.
- User correction: no separate Windows Start. Pairing plus local monitor selection
  enables background iPhone requests; Pause/Resume and Stop/unpair remain available.
- Track acceptance items with checkboxes; user integration checks stay open
  until the user reports results. Do not substitute additional test-writing.

## Summary

| Ticket | Implementation | Verification |
| --- | --- | --- |
| COMP-001 Protocol/core/capture libraries | Implemented (libraries only) | focused CI-like / mocked local (see `RUST_MIGRATION.md`) |
| COMP-002 Desktop eligibility & monitors | Implemented in source, integration open | not-run |
| COMP-003 Native shell & local controls | Background/no-taskbar controls built and installed | static/local Windows build; current interactive checks not-run |
| COMP-004 Selected-monitor capture | Implemented and connected to background owner requests | static/local Windows build; capture behavior not-run |
| COMP-005 Pairing & relay transport | Connected; cancellable async device transport | static/local Windows build; current end-to-end behavior not-run |
| COMP-006 Scheduling/triggers wiring | Background owner requests implemented; periodic/click scheduling pending | static/local Windows build; runtime not-run |
| COMP-007 Deliverable app | Development build installed; signing/release packaging pending | static/local Windows build and hash-matched install |
| COMP-008 Typed-input delivery (model types into chat boxes) | SendInput implementation connected and installed | static/local Windows build and hash-matched install; runtime not-run |
| Workspace manifest | Root workspace restored; reference-only subset retained | static/local Windows companion build using root Cargo.toml/lock |
| SRV-001 Rust relay crate | Implemented, not deployed | mocked local (pre-refactor); not-run since |
| SRV-002 API fast tiers / thinking levels | Implemented | not-run |
| IOS-001 iPhone Screen tab | Implemented; installed on iPhone 13 Pro Max | static/local device build; user-reported session + code; analysis/questions not-run |

## COMP-001 — Ship the existing protocol and policy foundations

- **Status:** Implemented (libraries only)
- **Scope:** `crates/codetether-companion-{protocol,core,capture}`.
- [x] Typed wire contracts and shared fixtures
- [x] Memory-only owner/device auth, session lifecycle, pairing budget
- [x] Paused-by-default bounded capture schedule; local-consent boundary
- **Evidence:** Prior focused CI-like and native Windows library runs recorded in
  `RUST_MIGRATION.md` (preserved artifacts under `target/companion-verification/`).
- **Limit:** Not a runnable Windows app; not yet consumed by `rust-shell`.

## COMP-002 — Native desktop eligibility and monitor selection

- **Status:** Implemented in source; integration and verification open
- **Scope:** `desktop-staging` (`bounds`, `error`, `monitor`, `platform`,
  `selection`, `native/`).
- [x] `check_available` / `validate_selection` exposed and used by the shell
- [x] Monitor enumeration with bounded count, duplicate rejection, panic-contained callback
- [x] Per-thread DPI guard restored on drop
- [x] Defined `WSF_VISIBLE` locally (not exported by windows-sys 0.61);
      crate now compiles on Windows (static/local)
- [ ] Windows run on real multi-monitor / locked / RDP desktops — **not-run (user)**
- **Excludes:** Pixel capture and GUI activation.

## COMP-003 — Runnable Windows shell and local controls

- **Status:** Local-only shell implemented (Pair/Start disabled); builds and launches on Windows
- **Scope:** `rust-shell` (sole binary `codetether-companion`), `src/native/*`.
- Present in source:
  - [x] Single-instance guard, per-monitor-v2 DPI setup, window class/creation
  - [x] Context/state ownership, panic-contained callback, failure propagation
  - [x] Control specs: monitor picker, Refresh, Pause, Stop/unpair, Exit, status,
        notice that closing hides to tray (not Stop)
  - [x] Pair, Start/resume, interval and trigger controls created **disabled**
  - [x] Monitor picker refresh/choose/clear; desktop recheck and WTS session
        invalidation (never restores consent)
  - [x] Tray icon add/remove guard; timer; DPI layout/fonts
  - [x] Command handling: Open, picker, Refresh, Pause, Stop (clears selection), Exit
  - [x] `dispatch.rs` message routing (close, command, timer, session, display,
        DPI, relayout, tray, `TaskbarCreated`)
  - [x] `messages.rs` message pump with dialog navigation
  - [x] `reveal.rs` show/hide; `WM_CLOSE` hides to tray and does not Stop
  - [x] `dpi.rs` initial and `WM_DPICHANGED` window sizing
  - [x] `menu.rs` tray menu (Open, Pause, Stop/unpair, Exit)
  - [x] `tray_events.rs` tray clicks and icon reinstall after Explorer restart
  - [x] Fixed `ids::TRAY` → `ids::TRAY_MESSAGE`; window class now registers
        `procedure::procedure` (was a nonexistent `callback::window_proc`)
  - [x] Added missing `picker::choose` (rechecks selection; never starts capture)
  - [x] Fixed WTS session-change constant imports (`UI::WindowsAndMessaging`)
- Open:
  - [x] Run in background button after pairing hides the controls without stopping sharing.
        WS_EX_TOOLWINDOW removes the taskbar button; minimize/close also hide.
        Notification-area tray access remains. Windows verification: not-run.
  - [ ] Tray tooltip does not yet change with state (always "capture OFF",
        accurate until capture exists)
  - [ ] Interactive control checks by the user (picker, Refresh, Stop, Exit,
        close-to-tray, tray menu, lock/unlock)
- Current behavior supersedes this historical shell baseline: Pair plus monitor selection enables background requests; the former Start control is Resume requests after explicit Pause.
- **Windows evidence (RILEYS-LAPTOP, `riley@192.168.50.135`):**
  - static/local: `cargo build -p codetether-companion-shell` (debug, rustc
    1.98.0) exit 0 → `C:\Users\riley\companion-build\target\debug\codetether-companion.exe`
    (368128 bytes). Source archive: `artifacts/companion-src.tgz`.
  - static/local launch smoke in interactive session 1 via `artifacts/smoke.ps1`
    (scheduled task, output `C:\Users\riley\companion-build\smoke.txt`):
    process stays running and responding; visible top-level window class
    `CodeTether.ScreenCompanion.Window`, title "CodeTether Screen Companion –
    capture OFF".
  - static/local single-instance: a second launch shows the "already running"
    notice dialog (`#32770`) instead of a second companion window.
  - not-run: clicking controls, tray menu, close-to-tray, session lock/unlock,
    DPI change, multi-monitor.
- **Verification:** User launches the app and checks control behavior — pending.

## COMP-004 — Selected-monitor image capture

- **Status:** Implemented in source (library only); host integration pending
- **Depends on:** COMP-002, COMP-003
- **Scope:** `desktop-staging/src/{capture,frame}.rs` and `src/native/capture/`.
- [x] Capture the selected monitor's exact physical rectangle with eligibility
      and selection rechecks before/after native work; no virtual-desktop union
- [x] Enforce 34M source pixels, bilinear resize without upsampling to a
      1600 px longest edge, and a hard 524288-byte JPEG writer bound
- [x] Retry JPEG qualities 75/55/35/20; fail rather than upload an oversized image
- [x] Memory-only DIB/RGB/JPEG buffers, redacted frame Debug, and RAII cleanup;
      the surface borrows its thread-bound device context
- [x] Per-operation cancellation checks; errors release owned resources
- [x] Use the Rust-1.85-compatible `jpeg-encoder` 0.6.1 and root lockfile
- [x] COMP-005/006: call after pairing/local monitor selection, cancel on
      pause/stop/session/display changes, and discard old worker results before upload
- **Safety limits:** Eligibility checks are snapshots, not synchronization.
  Cancellation is cooperative. Clearing adapter-owned buffers does not promise
  erasure of OS/driver or third-party encoder scratch buffers.
- **Verification:** not-run (no builds, tests, static checks or application runs).
  The root lockfile records `jpeg-encoder` 0.6.1 and `zeroize` for the desktop
  adapter; dependency entries alone are not compilation evidence.
  Background capture is connected in source; Windows execution remains not-run.

## COMP-005 — Pairing and authenticated relay transport

- **Status:** Implemented in source and connected to background requests
- **Depends on:** COMP-003
- [x] Pairing: code field + Pair button; `rust-shell/src/relay.rs` posts to
      `https://server.codetether.run/companion/pair` on a worker thread
      (10 s connect / 20 s total timeout, redirects refused); device token kept
      in memory only. Stop/unpair clears UI ownership and cancels the worker;
      worker-owned credentials are dropped after cleanup. Local monitor selection is also required.
      Evidence: static/local Windows build exit 0; **user-reported** live pairing
      from iPhone code succeeded on RILEYS-LAPTOP.
- [x] Authenticated command polling returns only an opaque, validated request UUID;
      owner questions never enter the Windows process
- [x] Frame upload uses the existing typed trigger/acknowledgement contracts,
      validates UUIDs and 512-KiB/1920px bounds, and zeroizes serialized image data
- [x] Fixed HTTPS origin, 10 s connect / 20 s total timeout, redirects refused,
      no-store requests, 8-KiB response bound, and redacted typed failures
- [x] Device token and response/request buffers are memory-only and zeroized;
      401/403/404/410 responses are classified as revocation
- [x] Stop/unpair clears the local capability without waiting for the relay;
      Exit also drops it, so relay availability cannot prevent local shutdown
- [x] COMP-006 calls polling/upload from one worker after pairing and local monitor
      selection, with cancellation, expiry checks and no parallel replacement worker
- [x] Correct device paths include `/companion/sessions/{id}/`; responses require exact 200/202 acknowledgement status
- **Verification:** not-run (no builds, tests, static checks, or live requests).
  Root lockfile dependency resolution completed offline with zero packages changed;
  that is configuration evidence only, not compilation evidence.
  Subsequent user direction: no Cargo commands here; any requested build is Windows-only.

## COMP-006 — Wire scheduling, triggers and session cancellation

- **Status:** Background owner-request path implemented in source; other triggers pending
- **Depends on:** COMP-004, COMP-005
- [x] One request-driven worker: two-second polling, one fresh capture per request,
      matching request-ID recheck before upload, five-second transient retry delay
- [x] No Windows Start required after pairing and local monitor selection
- [x] Native capture off the UI thread; HTTP futures cancelled on lifecycle changes;
      a replacement waits for old capture cleanup before polling/uploading
- [x] Lock/disconnect suspends; unlock restores unchanged selection automatically;
      explicit Pause stays latched until Resume requests
- [x] Stop invalidates pending pairing results; expired/revoked devices are forgotten
- [x] Visible sharing ON/OFF tray tooltip, title, and capture/upload status text
- [ ] Periodic / right-click / double-click triggers (controls remain disabled)
- [ ] User Windows/iPhone integration exercise — not-run
- **iOS Capture now:** `ScreenCaptureButton.swift` and `ScreenModel+Question.swift`
  use the existing fresh-frame request. Supplied session report cites signed build
  and install on iPhone 13 Pro Max: `mac:~/CodeTether-iOS-evidence/device-build-20261009T032421Z.log`.
  This is supplied evidence, not a build or device run by this Windows work session.
- **Verification:** not-run.

## COMP-007 — Deliver the Windows app for user integration testing

- **Status:** Development build installed; signed release/packaging still pending
- **Depends on:** COMP-003 – COMP-006
- [x] Static/local Windows build of the Rust binary using the repository root workspace
- [x] Per-user installation: `C:\Users\riley\AppData\Local\Programs\CodeTetherCompanion\codetether-companion.exe`
- [x] Desktop and Start menu shortcuts named **CodeTether Screen Companion**
- [x] Includes **Run in background**, no taskbar button, and background owner requests
- [ ] Signed release artifact, packaged installer/uninstaller and clean-machine checks
- [ ] User interactive/capture/typing integration checks — not-run
- **Evidence level: static/local.** Current host is `riley@192.168.50.187`,
  RILEYS-LAPTOP; it supersedes the previously unreachable `.135` address.
  Windows `cargo +stable build --locked -p codetether-companion-shell
  --bin codetether-companion` exited 0 using Rust/Cargo 1.98.0 and MSVC.
  `RUSTFLAGS=-Cdebuginfo=0` overrides the checkout's machine-specific Mold flag.
  Three unused-code warnings remain (`PAIRED`, `interval_seconds`, `revoked`).
  Build/install SHA-256: `8f2413921a5eac08217eebed55cd9a10047354bb08efef5f9b8526fb75981956`.
  Logs, source archive, executable and install receipt are retained in
  `artifacts/windows-build-20261009T034712Z/`; the old Windows build is untouched.
- **Not-run:** app launch, capture, tests, UI behavior and live relay checks. Installed
  artifact is unsigned. The older failed-connection evidence is preserved at
  `artifacts/windows-build-ssh-blocker.log` rather than overwritten.

## COMP-008 — Typed-input delivery: the model types into chat boxes

- **Status:** Implemented and installed (static/local Windows build and
  hash-matched installation); keyboard delivery runtime verification: not-run.
- [x] Consume the current relay/protocol reply contract, not invented endpoints:
      owner `POST /reply`, device `GET /commands` with `reply: { id, text }`,
      device `POST /typed` with `reply_id`. Capture questions remain opaque.
- [x] Real Win32 `SendInput` Unicode down/up events into the focused editable
      text field; no text generation, clipboard, DOM fill, UIA SetValue or Enter.
- [x] UIA checks editable/nonpassword/visible focus inside the selected monitor;
      no focus stealing, window activation or mouse automation.
- [x] Direct-at-caret typing with active/refused keyboard status, no scratch pad
      or preview window. Updated source; Windows build/install: **not-run**.
- [x] Pause, Stop/unpair, Exit and desktop/display changes cancel delivery.
      Focus/modifier changes or Escape interrupt; partial input is never retried.
- [x] Per-device attempted UUIDs prevent duplicate typing after ACK loss or
      Pause/resume. Capture and reply delivery share one serialized worker.
- [x] Owned reply buffers are zeroizing; no text logging or persistence.
- [x] Existing Swift `ScreenReplyView` provides manual **Type on Windows**.
- [x] Direct model handoff implemented in TS/Rust relay: an explicit typing
      request through Ask queues bounded text once after its matching fresh
      analysis succeeds, without an iPhone review/approval step. Replay,
      partial/error/periodic output cannot trigger typing; Pause clears queued
      replies. Direct vision remains `tools: []`.
- **Direct handoff verification: not-run** (builds, static checks, tests,
  deployment and interactive typing). Earlier Windows evidence below does not
  cover these relay and iOS changes.
- [ ] Detailed delivery outcome on iOS: the existing ACK only reports queue
      consumption (including refusal), not successful insertion into an app.
- [ ] User end-to-end typing check; unsupported accessibility providers fail
      closed. Single-line text only, <=2,000 UTF-16 units, no tabs/newlines.
- **Contract and limits:** [TYPING.md](TYPING.md). Focus checks are snapshots;
  already accepted keyboard events cannot be recalled.
- **Verification — static/local:** Windows build attempt 02 and installation
  exited 0. SHA-256 `dc9c80fd6f19febc031d7a06b785f9bc1834551531fdb248362c5e3beedbd0e6`.
  Evidence: `artifacts/windows-typing-20261009T040502Z/` (source archives,
  build logs/exit records, executable and installation receipt). Existing binary
  backed up; installed app left unlaunched. Tests/keyboard/UI/live relay: not-run.


## Workspace manifest

- [x] Rename the conflicting nested `./Cargo.toml` to `./workspace.Cargo.toml`.
      Keep it as a reference-only subset, including shared companion libraries;
      Cargo uses the existing root `../../Cargo.toml` and `../../Cargo.lock`.
- [x] Retain both Windows packages' existing root workspace registration and the
      sole `codetether-companion` binary in `rust-shell`.
- [x] Update `RUST_MIGRATION.md` and the native adapter's README.
- **Verification:** not-run for build/test/static checks. Root manifest and
  lockfile entries are source configuration, not Windows build evidence.

## Hosted inference

- **Relay deployed (live deployment):** `codetether-companion.service` active on
  `127.0.0.1:4099`; tunnel route `^/companion(?:/.*)?$` on `server.codetether.run`
  added with all other ingress unchanged. Relay suite 0 failures. Evidence:
  `artifacts/relay-deploy-20261009T021208Z/`. Public `POST /companion/sessions`
  now reaches the relay (owner token, empty body → 400 validation; previously 403
  from the main API); `/v1/models` still 200. User confirmed the iPhone Screen
  tab creates a session and shows a pairing code.
- [ ] `upstream.ts` still targets `http://127.0.0.1:4096`, not
      `https://server.codetether.run/v1/chat/completions`; analysis results unverified.

- **Status:** Blocked/out of Windows scope — inference stays on the relay
  (`https://server.codetether.run/v1/chat/completions`); Windows holds only the
  device capability. Relay wiring not re-verified here (not-run).
- Re-checked: `systemctl --user is-active codetether-companion` → `active`, a
  listener on `:4099` (static/local observation).

## IOS-001 — iPhone Screen tab (Swift/SwiftUI, `ios/Sources/Screen*`)

- **Status:** Implemented in source; built and installed on Riley's iPhone.
- [x] Independent Screen vision-model picker (`screen.model`); never changes Chat/Voice model
- [x] Session create, one-use pairing code with live expiry, native Windows consent guidance
- [x] Status view (paired/requested/analyzing/ready/paused/stopped), reconnect, session expiry
- [x] Streamed analysis (markdown text only, no screenshot pixels); typed-input
- [x] Ask about the screen: `POST /companion/sessions/{id}/request` (202 → `request_id`);
      typed or on-device dictated (never auto-sent); no automatic retry; blocked while busy
- [x] Lifecycle: tab/background pauses stream and dictation; 404/410/`stopped`/expiry
      clear local state; failed DELETE keeps session for retry; Stop confirmation
- [x] `ScreenResponseView.swift` removed (replaced by status/analysis views); README section
- **Evidence:**
  - static/local: sources synced to `mac:~/CodeTether-iOS`, `xcodegen generate`,
    `scripts/sign-from-vault.sh mac build` → `** BUILD SUCCEEDED **`
    (`mac:~/CodeTether-iOS-evidence/device-build-20261009T020503Z.log`, incremental).
  - live device install: `install-from-vault.sh mac 4745B27B-2B68-5164-A053-B869B07EE2CA`
    installed `run.codetether.ios` on iPhone 13 Pro Max; scripted launch denied
    (phone locked), so the connection receipt was not captured.
  - user-reported: Screen tab creates a session and shows a pairing code after relay deploy;
    pairing from Windows succeeded (see COMP-005).
- [ ] Live analysis stream with real frames — blocked on COMP-004/006 (Windows capture/upload)
- [ ] Ask flow end to end (needs Windows command polling + requested-frame upload)
- [ ] Dictation on device; existing `ios/Tests` do not cover questions — **not-run (user)**

## SRV-001 — Rust relay (`crates/codetether-companion-relay`)

- **Status:** Implemented in source; **not deployed** (TypeScript relay stays live
  until parity is shown).
- [x] Wire-compatible routes, owner/device split, frame/JPEG validation,
      cooldowns, 120-frame budget, fresh-frame requests, SSE snapshot/heartbeat/
      3-viewer limit, stop/revoke, direct streamed inference (`tools: []`),
      default upstream `https://server.codetether.run`, HTTPS-only, no redirects
- [x] SIGTERM/Ctrl-C stops every session before exit
- [x] Relay `src` files within the 50-line limit
- [ ] `tests/http.rs` is 55 lines (over limit); left as-is per "no more tests"
- [ ] systemd unit + deploy script for the Rust binary
- [ ] Parity check against the TypeScript relay, then cutover decision
- **Verification:** mocked local — relay unit, doc, and HTTP tests passed before the
  latest refactor; **not-run** since (user compiling).

## SRV-002 — Fast tiers and thinking levels on the API

- **Status:** Implemented in source; not compiled (user compiling).
- [x] `/v1/chat/completions` accepts `service_tier` and `reasoning_effort`,
      openai-codex only, validated against the tier/effort catalogs (400 otherwise)
      — `src/server/openai_model_options.rs`
- [x] `/v1/models` lists `service_tiers` / `reasoning_efforts` per model —
      `src/server/models_catalog/options.rs`
- [x] Streamed thinking emitted as `delta.reasoning_content`
- [ ] User compile + live request with `ultrafast` and `reasoning_effort`
- [ ] Check whether `cargo fmt` pushes `src/server/mod.rs` over the line ratchet
- **Verification:** not-run.

## Housekeeping

- [ ] Review `git diff --stat` for formatting-only churn from a workspace-wide
      `cargo fmt` run during SRV-001/002