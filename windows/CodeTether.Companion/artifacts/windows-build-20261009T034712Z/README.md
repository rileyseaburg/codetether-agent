# Windows companion build and installation

Evidence level: **static/local** (real Windows compilation and file installation).

- Host: `riley@192.168.50.187`, `RILEYS-LAPTOP`, user `rileys-laptop\riley`.
- Toolchain: Cargo 1.98.0, rustc 1.98.0, `stable-x86_64-pc-windows-msvc`.
- Visual C++ environment: VS 2022 BuildTools, x64 host and target.
- Source: uncommitted working-tree snapshot based on Git HEAD
  `76eaa1b74aad30e3827cf16fe67cd6dc250ca4ca`, not a clean commit build.
- Source SHA-256 (`source.tar.gz`):
  `fce3b126a0a4854dfb1f59e7369230d7b51ace1c937128034394d74aecba85f9`.
- Windows source workspace:
  `C:\Users\riley\companion-build\workspace-20261009T034712Z`.
- The archive preserves the root Cargo workspace and lockfile. No nested
  workspace or .NET reference application was built.

## Build

`Build-Windows.ps1` ran `cargo +stable build --locked -p
codetether-companion-shell --bin codetether-companion` in that root workspace.
`RUSTFLAGS=-Cdebuginfo=0` overrides the machine-specific Mold linker setting;
`CMAKE_GENERATOR=NMake Makefiles` uses the Windows native build tools.

`build-01.exit.txt` records **0**. `build-01.log` records the compiler output,
including three unused-code warnings (`PAIRED`, `interval_seconds`, `revoked`).

## Install

`Install-Windows.ps1` copied the 10,053,120-byte executable to
`C:\Users\riley\AppData\Local\Programs\CodeTetherCompanion\codetether-companion.exe`
and created Desktop and Start menu shortcuts named **CodeTether Screen Companion**.
`install-receipt.json` records paths, backup location, no launch, and no signature.

Built, installed and retrieved executable SHA-256:
`8f2413921a5eac08217eebed55cd9a10047354bb08efef5f9b8526fb75981956`.

**Not-run:** app launch, tests, capture, UI interaction or live relay checks.
The older build and unreachable-host evidence remain untouched.