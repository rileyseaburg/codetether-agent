# Windows keyboard-delivery development build

## Evidence level: static/local

Built on `RILEYS-LAPTOP` via `riley@192.168.50.187`, from the root Cargo
workspace in `C:\Users\riley\companion-build\typing-20261009T040502Z`.
`Build-Windows.ps1` initializes MSVC and runs:

```text
cargo +stable build --locked -p codetether-companion-shell --bin codetether-companion
```

`RUSTFLAGS=-Cdebuginfo=0` overrides the checkout's machine-specific Mold flag.
No Linux Cargo commands, tests or application/keyboard runs were performed.

## Preserved inputs and build records

- `source.tar.gz`: first uncommitted working-tree snapshot. SHA-256:
  `4db92bb3a5552ed374cb9dbb6415f2d93f05aa9e3e6ebd8e2683ad9aa2c8a5fc`.
- `build-01.log`, `build-01.exit.txt`: first build, exit 0; includes the
  subsequently corrected native-typing warnings.
- `source-02.tar.gz`: revised snapshot used for the installed build. SHA-256:
  `f6b95051a5b0af55c59406df2f805d782fde615f6a3d3dc80dc6abafd062aa0d`.
  Snapshots predate the final documentation/status updates.
- `build-02.log`, `build-02.exit.txt`: revised build, exit 0. Remaining warnings
  concern the existing unused PAIRED constant, interval_seconds field, revoked
  method, manifest semver metadata, and unused workspace patches.
- `codetether-companion.exe`: retrieved build artifact, 10,193,920 bytes.

## Installation: static/local

Installer exited 0 without launching the application or terminating processes.
`install-receipt.json` records the matching built/installed SHA-256:
`dc9c80fd6f19febc031d7a06b785f9bc1834551531fdb248362c5e3beedbd0e6`.

- Installed: `C:\Users\riley\AppData\Local\Programs\CodeTetherCompanion\codetether-companion.exe`
- Backup: `C:\Users\riley\companion-build\typing-20261009T040502Z\install-backup-20261008T230939`
- Desktop and Start menu: **CodeTether Screen Companion**.
- Installer source: `../windows-build-20261009T034712Z/Install-Windows.ps1`.
- Unsigned development executable, not a signed release package.

## Not-run

Actual keyboard delivery, cancellation/UI behavior, target-application insertion,
live relay compatibility, tests, signing and clean-machine installation checks.