# Agent, voice, images and saved chats

This replaces the earlier plain-completions client. Native sessions use
`POST /api/session`, `WSS /api/realtime/session/{id}`, and authenticated
session retrieval. The app does not imitate tools with a system prompt.

## Live deployment/API evidence

All paths below are under `../artifacts/`.
- `ios-agent-api/`: native agent invoked exec_command; actual output recorded.
- `ios-agent-websearch/tools.json`: actual websearch returned official Apple
  SwiftUI documentation links. Later empty-result searches are retained too.
- `ios-agent-image-retry2/tools.json`: image_gen succeeded with a real PNG.
  `generated-image.png` was downloaded through the authenticated image route.
  `provider-auth-kind.txt` records openai-codex OAuth selection, not credentials.
  The image tool succeeded and the PNG was retrieved, but the agent's later
  goal-verification step exceeded the overall test timeout; this is tool-level proof.
- `ios-media-live/media-proof.json`: anonymous upload401, authenticated upload
  and byte-identical download200, arbitrary file-read denial404.
- `ios-media-live/agent2/`: native image tool inspected the uploaded fixture
  and described its actual blue background and white circle.
- `ios-audio-live/`: public Kokoro now returns WAV200 rather than403.
- `ios-media-deploy/`: scoped Cloudflare ingress and enabled audio bridge.

## Physical iPhone evidence
Saved chats use server pagination and durable session IDs. Assistant text and
generated-image metadata are displayed before any later agent checks finish.

Mac evidence is in `~/CodeTether-iOS-evidence/`.
`device-live-test-20261005T113517Z.xcresult` records native chat and speaker tests.
`kokoro-playback-started.json` / `kokoro-playback-finished.json` record a 5.175s
Kokoro playback on the Speaker route at volume0.4, through normal completion.
Later UI runs extend this to saved-chat resume, photo picker, dictation start/stop,
and automatic assistant read-aloud; retain both failures and recovery runs.
Final recovery: `device-live-test-20261005T115459Z.xcresult` reports four tests,
zero failures: native tool chat + automatic audio + saved-chat resume after
app relaunch, photo picker, microphone start/stop, and speaker playback.
`agent-final-unit-tests.xcresult` reports26 tests, zero failures. Final copies,
screenshots, signed app and hash manifest are in `../artifacts/ios-agent-final/`.
The phone became unavailable afterward; an extra launch failed with CoreDevice1000.

## Security and limits

Bearer tokens stay in Keychain. Image uploads are JPEG-resized to1280px and sent
over authenticated HTTPS. Bridge storage is private and media reads are confined
to upload/generated-image roots; arbitrary paths and symlink escapes are denied.
