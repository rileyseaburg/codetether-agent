# Windows keyboard reply delivery

Implementation: Rust `desktop-staging/src/native/typing/` and
`rust-shell/src/native/work_reply.rs`. Verification is recorded separately below.

## Actual input, not text generation
- **No secondary review or approval screen.** In the iPhone's **Ask** field,
  start an explicit typing request with `Type`, `Enter`, `Write`, or `Fill`
  (optionally `Please` or `Can/Could/Would/Will you`). For example:
  `Type Bloomberg in the YouTube search field`.
- The relay waits for that request's matching fresh capture and successful
  model completion, parses one bounded final `windows-reply` JSON block
  (`text`, `target`), and queues its text directly once. The target description
  is informational; it never focuses or selects a Windows field.
- Partial/error analyses, periodic captures, screenshot instructions and
  replayed snapshots do not authorize typing. Invalid output or a busy queue
  reports no delivery, with no automatic retry. Pause discards pending replies.
- The iPhone's manual **Type on Windows** action remains available via the
  owner-authenticated `POST /companion/sessions/{id}/reply` route.
- Device-authenticated `GET .../commands` returns `reply: { id, text }` alongside
  the opaque capture request ID. Windows never receives the owner's question.
- Windows uses **Win32 SendInput**, `INPUT_KEYBOARD`, `KEYEVENTF_UNICODE`, and
  matching key-up events, including paired UTF-16 surrogates. No inference,
  clipboard, DOM fill, UIA SetValue, mouse actions, virtual hotkeys or Enter.
- Click a visible chat/text field on the selected monitor before sending from
  the phone. Windows types directly at its current caret, without opening a
  scratch pad/preview or imposing a preview countdown. It does not move focus,
  move the caret, or select text. An existing selection is replaced according
  to the target application's normal keyboard behavior.

## Safety and lifecycle
- UI Automation reads focus, editable capability, password/read-only state and
  bounds; it does not assign text. Unsupported accessibility providers fail closed.
- Only focused Edit/Document controls exposing writable Value or TextEdit
  patterns qualify, entirely inside the locally selected shared monitor.
- Single-line, nonblank text only, at most 2,000 UTF-16 units. Control characters,
  tabs and line separators are refused rather than potentially submitting a chat.
- Escape, modifiers, focus changes, lock/disconnect, monitor changes, Pause,
  Stop/unpair and Exit stop delivery. A 30-second deadline is checked between
  bounded UIA calls. No automatic elevation or bypass of Windows UIPI.
- Desktop/focus checks are snapshots, not an atomic lock with SendInput. Already
  accepted events cannot be recalled; any failure may leave partial text.
- Replies run serially with captures. Attempted UUIDs are retained (up to 4,096)
  per paired device, including across Pause/resume. Lost ACKs retry only the ACK,
  never the keystrokes. Restart/unpair forgets credentials and the UUID set.
- `POST .../typed` sends `{ reply_id }` after handling, including refusal. Its
  existing `typed` boolean means queue consumption, NOT proof of insertion.
  The current protocol has no detailed result or reply-expiry timestamp.
- Supplied text stays in memory; owned text buffers are zeroizing. No text is
  logged or copied into a separate window. Companion status and Pause/Stop remain
  available. Windows/UIA/application/driver copies cannot be guaranteed erased.

## Verification
- Direct-at-caret/no-preview change: **not-run** for Windows build, tests and
  installation. This source change does not alter the already-running binary.
- Earlier Windows build/install evidence is recorded in the COMP-008 delivery
  ticket; it does not establish this direct model-to-reply handoff.
- Direct handoff source changes: TypeScript and Rust relay plus SwiftUI copy and
  removal of the review UI. Builds, static checks, tests, deployment and interactive
  typing for this change: **not-run**. Existing evidence artifacts are unchanged.