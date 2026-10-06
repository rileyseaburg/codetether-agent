# Camera attachment update — 1.5.0 (9)

Install from Safari on Riley's registered iPhone:
<https://ios.codetether.run/releases/1.5.0-9/index.html>.
This is an Ad Hoc update for iOS 17+; only provisioned devices can install it.
Update the existing app in place; do not delete it to install this update.

## Behavior

- The camera button adds a photo to the existing message attachment flow.
- Permission denial offers Settings; restricted/unavailable cameras offer Photos.
- Cancelling capture leaves the draft and existing attachments unchanged.
- Capture is disabled while busy, checking permission, or at three attachments.
- Photos are resized to at most 1280 pixels and JPEG-encoded to at most 4 MiB.
- Authentication is not provisioned through the IPA or installation page.

## Retained evidence (paths relative to the agent repository)

- **Focused CI-like (simulator):** 47 tests, zero failures, exit 0;
  `ios/evidence/camera-1.5.0-9/test.log` and `exit-status.txt`.
  Mac result: `~/CodeTether-iOS-evidence/camera-tests-20261005T172913Z/tests.xcresult`.
- **Static/local:** exported IPA signature, version, registered-iPhone profile,
  debugging disabled, and camera usage description:
  `artifacts/ios-camera/1.5.0-9/upgrade-signature-verification.json`.
- **Static/local:** old/new bundle ID, application identifier, and Keychain groups
  match in `artifacts/ios-camera/1.5.0-9/installed-signature-verification.json`
  and `upgrade-signature-verification.json`. This supports an in-place upgrade;
  it does not establish saved-login reuse on Riley's phone.
- **Static/local:** NAS copy and per-file checksums:
  `ios/evidence/camera-1.5.0-9/nas-publication.json`;
  Mac folder: `~/NAS/CodeTether-iOS-1.5.0-9`.
- **Static/local:** expanded exact-value Vault-secret audit, exit 0: eight files,
  two decompressed IPA/ZIP archives, five credential values, no matches:
  `artifacts/ios-camera/1.5.0-9/audit-expanded/vault-secret-audit-final.json`
  and `vault-secret-audit-final-exit.txt`. Archive and Vault errors fail closed
  without exposing captured contents. The earlier five-file audit is retained at
  `artifacts/ios-camera/1.5.0-9/deployment/secret-audit.txt`.
  This scan is not a comprehensive proof that every possible secret is absent.
- **Static/local and mocked local:** seven Node regression tests, exit 0:
  `artifacts/ios-camera/1.5.0-9/audit-expanded/node-tests.log`
  and `node-tests-exit.txt`; includes compressed-secret and malformed-archive
  fixtures, release configuration, and mocked immutable-release asset routing.
- **Live deployment:** HTTPS 200 for page, manifest, and checksum-matching IPA:
  `artifacts/ios-camera/1.5.0-9/audit-expanded/https-final/https-verification.json`.
- **Live deployment:** old 1.4.0 (8) download retains its pinned checksum:
  `artifacts/ios-camera/1.5.0-9/previous-release-handoff/https-verification.json`.

IPA SHA-256: `b5590ac5751a5c07b7f6d88ef2aac52051419ac17eca476a388500e8f663ec32`.

## Not-run: physical iPhone confirmation

amac reports Riley's iPhone as unavailable. On-device capture, saved-login reuse,
photo/file attachment regression checks, and audio playback remain unconfirmed.