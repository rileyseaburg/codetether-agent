# Windows typing build — 2026-10-09T04:13:42Z archive

Owner-reply keyboard delivery (iOS "Type a reply on Windows" → relay → Windows types
into focused input → ack) built and installed on RILEYS-LAPTOP.

## Build

- Host: `riley@192.168.50.135` (RILEYS-LAPTOP; adapter flipped mid-session from
  192.168.50.187 — use whichever is reachable, or the router DHCP reservation fix).
- Archive: `source.tar.gz` (5,139,016 bytes, 10,130 members, prefix
  `windows/CodeTether.Companion/`), sha256
  `050d09f01782421c331e9821b559eb344a666bf3a0b9f788ca30432b2b2ed6ad`.
  Staged locally at `/home/riley/ctb-20261009T041342Z/` and copied here.
- Windows workspace: `C:\Users\riley\companion-build\workspace-20261009T041342Z`.
- `Build-Windows.ps1` attempt 01: `cargo +stable build --locked
  -p codetether-companion-shell --bin codetether-companion` (rustc 1.98.0,
  MSVC; `RUSTFLAGS=-Cdebuginfo=0`, `CMAKE_GENERATOR=NMake Makefiles`).
  Exit **0** in 8m 41s. Log tail + exit file in `evidence/`.
- Warnings (3, pre-existing, same set as the 034712Z build): dead code
  `PAIRED` (ids.rs), `interval_seconds` (relay/device.rs), `revoked`
  (relay/error.rs). No errors.
- Binary: `target\debug\codetether-companion.exe`, sha256
  `7153C6617F87CECEFF0499C31A63234C0593822B1B55545B86A52E0D36A40D3E`.

## Install

- `Install-Windows.ps1` (copied from the 034712Z artifacts; unchanged) at
  `C:\Users\riley\companion-build\workspace-20261009T041342Z`.
- Installed to `C:\Users\riley\AppData\Local\Programs\CodeTetherCompanion\codetether-companion.exe`
  (10,193,920 bytes), build/installed hash match verified by the script.
- Shortcuts refreshed (Desktop + Start Menu). Previous exe backed up to
  `install-backup-202601008T233324` on the Windows host.
- Not launched (`launched: false`); not code-signed (`signed: false`).

## Scope

This archive contains the complete typing feature: TS relay `replies.ts`
(queue/ack/expire, 60 s TTL), Rust `codetether-companion-relay` `replies.rs`
parity, `codetether-companion-protocol` `reply.rs` (`DeviceReply`, `TypedAck`,
`ReplyReceipt`, `ReplyRequest`) + `DeviceCommand.reply`, iOS `ScreenReply*`
files, `rust-shell` `work_reply.rs` phases 4–7, `desktop-staging` native
typing (`type_in_focused_input`, UIA editable validation, SendInput Unicode).

## Not run (by user instruction)

No tests, static checks, app launch, pairing, capture, typing session, or any
runtime behavior — verification of runtime typing remains user-owned.
