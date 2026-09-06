# Image delivery to models and child agents

Images must be model input, not a filename, a text placeholder, or base64 buried
in ordinary tool prose. The original tool call must retain ownership of its
image output.

## Codex reference

The transport contract was compared directly with OpenAI Codex commit
[`e01f38c388f4907f02ac5b4980a37487686204c8`](https://github.com/openai/codex/commit/e01f38c388f4907f02ac5b4980a37487686204c8):

- `codex-rs/protocol/src/models.rs`: `FunctionCallOutputBody` and
  `FunctionCallOutputContentItem::InputImage`.
- `codex-rs/core/src/tools/handlers/view_image.rs`: images are emitted in the
  originating `FunctionCallOutput`, not as a new user request.
- `codex-rs/core/tests/suite/view_image.rs`: checks call-ID association and
  array-valued image output.
- `codex-rs/protocol/src/user_input.rs` and `multi_agents_common.rs`: typed
  image and local-image input. Upstream MAv1 supports structured child items;
  its newer MAv2 spawn API is text-only. These are distinct interfaces.

## Tool result pipeline

1. Image producers attach `image_data_url` metadata, containing an object or an
   ordered array of `{ "data_url": "data:image/png;base64,...", "mime_type": "image/png" }`.
2. Serial sessions, parallel tool batches, legacy agent execution, and swarm
   execution retain these attachments as `ContentPart::Image` beside the
   originating `ContentPart::ToolResult` in the same `Role::Tool` message.
3. Text feedback, truncation, and RLM routing process text separately; image
   payloads do not pass through those text transformations.
4. Provider conversion serializes the actual image input. Codex Responses HTTP
   and WebSocket requests use `function_call_output` with `call_id` and an
   `output` array containing `input_image` and `input_text` entries.

Browser and Windows screenshots attach the captured PNG buffer directly while
retaining their file paths and coordinate metadata. They do not reread a shared
screenshot file later to reconstruct the image. MCP adapters preserve image
blocks in both directions, use standard `mimeType` on the wire, and retain
error status. Image data from MCP is already base64 and is not encoded twice.

## User and child input

Image-bearing user messages preserve ordered text/image content in provider
requests. `send_input.items` exposes the actual typed fields, including `text`,
`image_url`, and `path`. Relative `local_image` paths resolve against the trusted
parent workspace before the bytes are placed in the durable child mailbox.
Explicit HTTP(S) image references are forwarded without fetching them locally;
the receiving provider must support that reference type.

For example, send the image explicitly rather than only naming it in prose:

```json
{
  "target": "child-agent-id",
  "items": [
    { "type": "text", "text": "Inspect this screenshot." },
    { "type": "local_image", "path": "artifacts/screenshot.png" }
  ]
}
```

## Provider-specific boundaries

- Anthropic/Vertex and Bedrock nest supported image blocks within the matching
  native tool result. Unsupported formats/references produce visible notices.
  Vertex and Bedrock require embedded base64 sources; remote HTTP(S) references
  are supported by direct Anthropic and compatible Chat/Responses APIs. Use
  `local_image` when the recipient needs portable embedded pixels.
- Chat Completions permits images in user content, but not in tool reply
  content. OpenAI-compatible adapters retain the text replies and add labeled
  image companions after the entire consecutive tool-reply batch. This is a
  transport-only adaptation, not a stored user turn or native Responses parity.
- Packed tool histories associate an image with its preceding result. The
  normal runtime records one result per message; images are never shared across
  distinct calls merely because they appeared in the same execution batch.
- A text-only transport cannot gain vision from serialization changes. Gemini
  Web emits an explicit unsupported-image notice instead of silently dropping
  the attachment. Image detail/resizing behavior is not changed by this fix.