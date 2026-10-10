# iPhone update — 2026-10-09

The user explicitly requested installation of the previously simulator-tested
Screen session repair. This superseded the no-reinstall/no-relaunch constraint
for the iPhone update only. Windows and the relay were not restarted.

## Build — static/local

- Xcode 16.4 built the existing Swift app, scheme `CodeTether`, for generic iOS.
- Source: Mac `~/CodeTether-ScreenMultiTurn-20261009T055645Z/`, matching the local
  `source.sha256` manifest. The preceding simulator evidence is in
  `../screen-multiturn-20261009T055645Z/` (mocked local, not live typing proof).
- Bundle `run.codetether.ios`; version 1.5.1; build 23 (build-setting override).
- `build-device.exit.txt`: 0. `codesign --verify --deep --strict` exited 0.
- Signature retains application identifier `J9YRM3U37D.run.codetether.ios`.
- Archive: `CodeTether-1.5.1-23.zip`.
- Archive SHA-256, matching Mac and retrieved local archive:
  `1cde0925847ca0172098012e34f2e1d3ec2ff282ce3abc3e4ec6789a08ba18e2`.
- Mac build/output: `~/CodeTether-iOSInstall-20261009T060053Z/`.

## Installation — real platform upload (direct device install, not App Store)

`devicectl device install app` exited 0; `install.json` reports success on the
paired iPhone 13 Pro Max, CoreDevice ID
`4745B27B-2B68-5164-A053-B869B07EE2CA` (Xcode UDID
`00008110-001879C23E0A401E`). `installed-after.json` independently reports
`run.codetether.ios` version 1.5.1, build 23. The app was updated in place, not
uninstalled. No bearer/bootstrap replacement or credential reset occurred.

## Launch and connection — real platform upload follow-up

The existing `verify-device.rb` refused pending bootstrap credentials, then
launched the installed application. `relaunch-20261009T060216Z/launch.json`
records launch success. However, the bounded connection-receipt check exited 1:
`No fresh authenticated receipt after Keychain-only relaunch`.
The retained receipt is dated `2026-10-09T05:17:19Z`, before this launch, so it
is not evidence of current authentication. The root cause is unresolved; this
failure does not negate the observed installation or establish a token failure.
Retain `launch-verification.log`, `launch-verification.exit.txt`, and the full
`relaunch-20261009T060216Z/` evidence directory.

No capture/typing request was submitted. Live multi-turn and Windows insertion
verification are not-run for this build. The Screen session and drafts are
memory-only; the restart loses the phone's in-memory pairing and requires a new
Screen pairing. Saved app credentials were not changed, but their successful
reuse has not been established by the failed fresh-receipt check.
