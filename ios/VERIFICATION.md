# Verification — 2026-10-05

Evidence root: `../artifacts/ios-client/` (relative to this directory).
Remote evidence: `amac:~/CodeTether-iOS-evidence/` using `ssh-amac.conf`.
Earlier failures are retained alongside recovery runs.

## Static/local — source and signed build

- Xcode 16.4 on the paired Mac Mini, macOS 15.3.1.
- Final Vault-authorized signing ran through the project-local `amac` alias.
- `mac-evidence/device-build-20261005T032924Z.log`: `BUILD SUCCEEDED`.
- `mac-evidence/codesign.txt`: team `J9YRM3U37D`, Apple Development identity.
- `CodeTether.app/`: signed installable app; bundle `run.codetether.ios`.
- `mac-evidence/build-manifest.json`: 13 Swift source/test hashes and seven
  product-file hashes; local files matched all hashes with `sha256sum -c`.
- `secret-audit.txt`: exact-value scan of Vault bearer and private keys found
  no matches across source and retained build/evidence files.

## Mocked local / static-local Keychain tests

`xcodebuild test -project CodeTether.xcodeproj -scheme CodeTether`
with the dedicated iOS 18.6 simulator, signing enabled, and result bundle
`simulator-tests-5.xcresult`: **10 tests, zero failures**.
`test-summary.json` contains xcresulttool's machine-readable summary.
HTTP tests use fixtures; Keychain tests exercise actual simulator storage.
Unsigned simulator tests failed Keychain access; ad-hoc signing corrected it.
The temporary simulator was removed after preserving all xcresult bundles
to recover space on the build host. No existing simulator was removed.

## Live deployment/Argo — direct physical iPhone deployment (no Argo)

- Device: Riley's iPhone 13 Pro Max, `4745B27B-2B68-5164-A053-B869B07EE2CA`.
- `mac-evidence/device-install-final.json`: successful final installation.
- `mac-evidence/relaunch-20261005T033010Z/launch.json`: successful app launch.
- Same directory's `documents-before.json`: no pending bootstrap token file.
- Same directory's `connection-receipt.json`: authenticated Keychain-only
  refresh at `2026-10-05T03:30:11Z`, server `4.7.6-dev.23`, three visible agents.
- `live-http.txt`: health 200; anonymous protected requests 401;
  Vault-authenticated version and agent requests 200.
- App Store/TestFlight publishing: **not-run**, not requested.
- A second physical-device Keychain-only check also succeeded at
  `03:38:37Z`; evidence: `mac-evidence/relaunch-20261005T033834Z/`.

## Independent goal-audit limitation

The verifier accepted implementation/hash evidence but could not reach Vault
or SSH because its environment lacks Vault authentication and denies network.
Parent rechecks still succeeded: `parent-access-recheck.txt` confirms signature
and 10-test xcresult; `secret-audit-recheck.txt` reports 2,437 clean files.
