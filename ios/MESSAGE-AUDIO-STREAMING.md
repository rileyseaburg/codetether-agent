# Per-message audio and bounded stream presentation — 1.4.0 (8)

Every text message has a Kokoro speaker control. The active message switches
to Stop reading; choosing another message stops the previous playback.
Manual playback does not require the automatic Read aloud toggle to be on.
Changing conversations stops playback from the previous conversation.

The transport is the native agent WebSocket (not a direct SSE connection).
Token-level deltas and unknown telemetry are discarded during decoding.
Decoded frames are processed off the main actor. Completed assistant items
update one stable reply bubble per turn, including the terminal response.
The replay cache is bounded at128 IDs. Raw tool activity is no longer shown
as a second stream of chat content; only the normal Working indicator remains.

Rendering initially shows the latest40 messages, with Load earlier messages.
Long messages initially render12000 characters with Show full message. These
are presentation windows, not deletion: server history remains unchanged.
Reasoning text and full tool logs are not retained in the display snapshot.
Restored assistant revisions collapse to the latest response per user turn.
Image previews are downsampled to1024px and released when they leave view.

## Verification

`StreamMemoryTests` exercises100000 ignored token events,2000 completed reply
events, bounded replay storage, and XCTest memory measurements. Large frame
tests cover1MiB token/image payloads without retaining their contents.
`MessageAudioTests` covers selected-message playback and immediate stop.
`ImageThumbnailTests` checks bounded decoded image dimensions.
The device UI test plays, stops, and replays an individual message via Kokoro.
See `../artifacts/ios-message-audio/` for retained run evidence and build.

Simulator metrics are local microbenchmarks, not proof of a leak-free device
under every workload. Physical-device playback/memory verification requires
