# CodeTether for iPhone

Native SwiftUI client for `https://server.codetether.run` (iOS 17+).
Opens to the actual CodeTether agent, with tools, streaming status and replies.
Saved chats are persisted by the server; open the clock icon to browse and resume.
Add image opens the photo picker with removable previews; pasting an image
into the message field attaches it the same way. Dictate transcribes
speech into an editable draft. Read aloud and Test speaker use server-side Kokoro.
Server status remains a second tab. Settings manages the device-only bearer.
See `AGENT-VOICE-VERIFICATION.md` for the current feature evidence.

## Screen tab (Windows companion)

Screen analysis and directly requested typing, separate from tool-capable chat.
Choose a Screen vision model (independent of the Chat model), instructions and
interval, then tap **Pair Windows device** for a one-use code. On Windows,
enter the code in CodeTether Companion and select a monitor; no separate Start
step is required. Pairing alone does not select a capture scope. **Ask**
(typed or on-device dictated, never auto-sent) requests one fresh frame via
`POST /companion/sessions/{id}/request`.

To type, focus the intended Windows field and ask, for example, **Type Bloomberg
in the YouTube search field**. Requests start with `Type`, `Enter`, `Write`, or
`Fill` (optionally `Please` or `Can/Could/Would/Will you`). Successful fresh
analysis queues valid keyboard text directly on the relay, with **no review or
second confirmation**. The relay reports queued/not-queued in the analysis;
queued does not prove insertion. Windows still checks focus and supports
Pause/Stop; there is no clicking, Enter, submission, or automatic retry.
Manual **Type on Windows** remains available separately.

Leaving the tab pauses the stream, not an already-authorized relay task;
**Stop session** revokes pairing. No screenshot pixels are shown or stored.
Direct typing handoff builds, checks, tests, deployment and device use: **not-run**.
Relay contract: `../scripts/public-server/companion/`. [Multi-turn session repair and simulator evidence](SCREEN-SESSION-VERIFICATION.md).

## Source of truth

- `../src/server/mod.rs`: `GET /api/version`, `GET /api/agent`.
- `../src/server/session_routes/` and `session_realtime/`: persisted agent turns.
- `../scripts/public-server/audio/`: authenticated Kokoro and image bridge.
- `../src/server/version_info.rs`: version response.
- `../src/agent/types.rs`: agent response.
- `../src/server/auth/`: mandatory `Authorization: Bearer …` middleware.

## Build and test on the Mac

Requires Xcode, XcodeGen, SSH access, and authorized local Vault access.
Sync this directory to `~/CodeTether-iOS/` on the build Mac, then run:

```sh
cd ~/CodeTether-iOS
xcodegen generate
xcodebuild test -project CodeTether.xcodeproj -scheme CodeTether \
  -destination 'platform=iOS Simulator,id=SIMULATOR_UUID' \
  -derivedDataPath build -resultBundlePath /path/to/unique-tests.xcresult
```

Do **not** disable signing for simulator tests: the Keychain tests require
the application-identifier entitlement from ad-hoc signing.
Run tests in a dedicated simulator; they replace that app's Keychain token.

From the Vault-authorized workstation, run `bash scripts/sign-from-vault.sh HOST`,
then `bash scripts/install-from-vault.sh HOST DEVICE_UUID`. Keep the phone unlocked.
The scripts default to `amac`. Initially that name did not resolve; the paired
Mac Mini was accessible as `mac` (192.168.50.249). `ssh-amac.conf` supplies a
project-local alias without changing global SSH settings. From this directory:
`export SSH_CONFIG="$PWD/ssh-amac.conf"`, then run the scripts with host `amac`.
Direct access: `ssh -F "$SSH_CONFIG" amac`. Verify the trusted host key first
when using a new workstation; strict host-key checking remains enabled.
Signing team: `J9YRM3U37D`; bundle ID: `run.codetether.ios`.
Detailed credential handling and evidence are in `SECURITY.md` and `VERIFICATION.md`.

## Rechecking an installed app

Run `ruby ~/CodeTether-iOS/scripts/verify-device.rb DEVICE_UUID` on the Mac.
It refuses a pending bootstrap file, relaunches without providing credentials,
and requires a fresh authenticated receipt from the physical app container.
This proves saved-Keychain reuse, not just successful installation.
Build and device evidence is preserved in `~/CodeTether-iOS-evidence/`.
