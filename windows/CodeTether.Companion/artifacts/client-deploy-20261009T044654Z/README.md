# iOS and Windows client delivery — 2026-10-09

User requested builds and installations of both current clients.
Build/signature/hash evidence: **static/local**. Installation receipts are
retained separately and do not establish runtime behavior or platform upload.

Source snapshots contain uncommitted changes at base commit
`76eaa1b74aad30e3827cf16fe67cd6dc250ca4ca`; not a clean-commit build.

## iOS — built and installed

- Host: `mac`, Xcode 16.4; `ios/scripts/sign-from-vault.sh mac build` exited 0.
- `ios/device-build-20261009T044709Z.log`: `** BUILD SUCCEEDED **`.
- Bundle: `run.codetether.ios`, version 1.5.1 (22), Apple Development signed.
- iPhone 13 Pro Max: `4745B27B-2B68-5164-A053-B869B07EE2CA`.
- Device install exited 0; `ios/install.json` reports `success`.
- Installed container: `A318A745-00A9-4918-8867-DC2E810BEA53`.
- `ios/CodeTether.app.zip` preserves the signed bundle and debug dylib.
- Existing credentials preserved; no bootstrap injection or application launch.
- Source SHA-256: `db97b697c5d7d46226b2cb341d00fcee94bc407a3d1063e0928f5b43ed67781d`.
- App ZIP SHA-256: `c250e5c299b606d6a164dadd1b1626b51d84ae4358eecb520d8072e31016be46`.
- Debug dylib SHA-256: `79c56ed0d1fe306449375cdc11cba38258848fb799692e58338e62e1a5d9fbd0`.

## Windows — built and installed

- Host: `riley@192.168.50.135` (`RILEYS-LAPTOP`).
- Workspace: `C:\Users\riley\companion-build\workspace-20261009T044654Z`.
- Command: `cargo +stable build --locked -p codetether-companion-shell --bin codetether-companion`.
- `windows/Build-Windows.ps1` builds using MSVC on Windows, not Linux.
- Source SHA-256: `99359447c722c4709a7458932ad35bb37d96ee14bf95778d60493eb4e39de2b3`.
- Build exited 0 and finished in 8m 52s (development profile).
- Three unused-code warnings: `PAIRED`, `Device.interval_seconds`, `Error.revoked`.
- Executable SHA-256: `881c13e57ad09667ecc3687075a549dff244a791ed5cabac3f26fe347eea3fe0`.
- Installed executable hash matches the build; size: 10,193,920 bytes.
- Installer exited 0; Desktop and Start Menu shortcuts refreshed.
- Previous files backed up under the workspace's `install-backup-20261008T235755`.
- Receipt reports `launched: false`, `signed: false`; capture consent unchanged.
- Evidence: `windows/build-01.log`, `build-01.exit.txt`, `source-hash.txt`, `install-receipt.json`, and `codetether-companion.exe`.
- Target: `C:\Users\riley\AppData\Local\Programs\CodeTetherCompanion\codetether-companion.exe`.

## Not-run

Tests, application launches, capture/typing checks, relay deployment and end-to-end integration.
The new model-to-typing handoff also requires the updated relay process;
installing these clients alone does not deploy the relay.