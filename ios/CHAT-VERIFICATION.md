# Chat-enabled iPhone app — 1.1.0 (3)

The earlier 1.0 app was a status/agent catalog, not chat. This update adds the
requested native chat UI and makes Chat the launch tab. It uses authenticated
POST `/v1/chat/completions`, sending full conversation context with each turn.
Model choices come from `/v1/models`; selections persist between launches.
Conversations themselves stay in memory. Stop cancels the current request;
failed sends restore the draft rather than duplicating the user message.

## Live deployment/Argo — direct physical-device UI test, no Argo

`CodeTetherLive` XCUITest ran on Riley's iPhone 13 Pro Max, device
`4745B27B-2B68-5164-A053-B869B07EE2CA`, using the existing Keychain token.
The test opened CodeTether Chat, typed a prompt into `chat-input`, tapped
`chat-send`, and observed a nonempty `assistant-message`. It attached an
actual phone screenshot. The observed test run finished with TEST SUCCEEDED.

Remote evidence: `~/CodeTether-iOS-evidence/` on the paired Mac:
- `device-live-test-20261005T105520Z.log` and corresponding `.xcresult`.
- `chat-live-attachments/` contains the real-device screenshot and manifest.
- `physical-chat-receipt.json` records the actual model and response timestamp.

The server's default `bedrock/us.anthropic.claude-fable-5` returned HTTP 500
with Bedrock's `403: Bearer Token has expired`. No global credential was changed.
`openai-codex/gpt-5.5` returned a real answer and is the app's initial choice
when advertised by the server. User selections take precedence; no silent
cross-provider retry occurs. Local evidence is under `../artifacts/ios-chat/`.
