# Physical iPhone reply-error observation

## Scope and result

The user reports Ask/capture working but no Windows text from Type a reply.
The old manual bridge sent through **Ask** (`screen-ask`), not through the
direct reply button. That run is not a direct `/reply` delivery test.

**Live physical-device observation:** runs 04 and 05 each executed one
observation-only XCTest with zero failures and xcodebuild exit 0. These
results establish observation, not successful requests, inference or typing.

Run 04 found the reply editor visible, its Send button disabled, and both:

- `Connection unavailable. Check Server settings and try again.`
- `Analysis failed. Check the selected vision model and try again.`

Run 05 confirmed the connection message belongs to `screen-error`, the
global session/stream error. It also found `Stream paused. Reconnect or Stop.`
The analysis message remains separately visible. No new request was sent.

`replyError: []` is **not** proof that the direct reply had no error: current
`ScreenReplyView` does not assign `screen-reply-error` to its error text.
The allowlisted page-wide observations cannot recover an earlier overwritten
direct-reply error or its HTTP/transport status. Root cause remains unresolved.

## Preserved evidence

- `reply-observation-04.json`, `.log`, `.exit.txt`, `.xcresult.tar.gz`
- `reply-observation-05.json`, `.log`, `.exit.txt`, `.xcresult/`
- `ReplyObservation.swift`, `ReplyErrorLocation.swift`, `ReplyErrors.swift`

The runner targeted hardware UDID `00008110-001879C23E0A401E` through the
existing Mac connection. Only a separate observation runner was installed;
the owner app was not relaunched, reinstalled or stopped, its credentials
were not changed, and the relay was not restarted. No reply, capture,
reconnect, Stop or Windows keyboard input was initiated by these observations.