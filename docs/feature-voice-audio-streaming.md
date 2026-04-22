# Feature Request: Voice Audio Streaming & Playback

## Requested By
Spotless Bin Co project — Riley

## Summary
Add a new action `speak_and_play` (or enhance the existing `speak` action) to the `voice` tool so that generated audio can be streamed/played back to the user directly, rather than only saving a WAV file to disk.

## Current Behavior
The `voice` tool (`src/tool/voice.rs`) already calls the Qwen TTS API and generates speech. The `speak` action:
1. Sends text to `POST /voices/{voice_id}/speak` on the Voice API
2. Receives a WAV response
3. Saves it to disk as `voice_{job_id}.wav`
4. Returns a text result with the file path

**Problem**: The user has to manually open the WAV file to hear it. There's no integrated playback.

## Desired Behavior
After generating speech, the audio should be **automatically played back** to the user through one of these mechanisms (in priority order):

### Option A: Serve via local HTTP and open browser (preferred)
1. Generate the audio as currently done
2. Serve the WAV file from a temporary local HTTP endpoint (e.g. `http://localhost:{port}/audio/{job_id}.wav`)
3. Open the user's default browser to a simple HTML page with an `<audio>` player that auto-plays the WAV
4. This works cross-platform and requires no native audio dependencies

### Option B: Native audio playback
1. Use a Rust audio crate (e.g. `rodio`) to play the WAV through the system audio output
2. Simpler but adds a native dependency and won't work in headless/server environments

### Option C: Return a data URI / base64 HTML page
1. Embed the audio as a base64 data URI in a self-contained HTML page
2. Write it to a temp file and open the browser
3. No server needed, but large scripts produce huge HTML files

## API Reference
The Qwen TTS Voice API runs at:
- **Internal**: `http://localhost:8015`
- **Public**: `https://voice.quantum-forge.io`
- Configured via `CODETETHER_VOICE_API_URL` env var

### Key endpoint used: `POST /tts/speak` (JSON)
```json
{
  "script": "Text to speak",
  "voice_id": "960f89fc",
  "language": "english"
}
```

**Response**: WAV audio bytes streamed back.

### Alternative: `POST /voices/{voice_id}/speak` (multipart form)
Currently used by `voice.rs`. Returns WAV audio with `X-Job-Id` header.

### Output retrieval: `GET /outputs/{job_id}`
After generation, audio is also available at this endpoint (useful for streaming from the API directly instead of proxying).

## Implementation Notes

### New action: `speak_stream`
Add a new action to `Params.action` enum: `"speak_stream"`

```
action: "speak_stream"
text: "Text to speak"
voice_id: "960f89fc" (optional)
language: "english" (optional)
play: true (optional, default true — auto-play in browser)
```

### Option A Implementation Sketch (recommended)

```rust
// In voice.rs or a new voice_stream.rs module:

// 1. Call /tts/speak (JSON) to generate audio
// 2. The API returns job_id + saves the file server-side
// 3. Construct a streaming URL pointing to the Voice API
// 4. Generate a minimal HTML page with an <audio> element
// 5. Open it in the default browser using open::that() or similar

async fn speak_stream(&self, params: &SpeakParams) -> Result<ToolResult> {
    // POST /tts/speak to get job_id
    let resp = self.client.post(format!("{base_url}/tts/speak"))
        .json(&json!({
            "script": &params.text,
            "voice_id": voice_id,
            "language": language,
        }))
        .send().await?;

    let body: Value = resp.json().await?;
    let job_id = body["job_id"].as_str().unwrap_or("unknown");
    let output_url = body["output_url"].as_str().unwrap_or("");

    // Build HTML player page
    let html = format!(r#"<!DOCTYPE html><html><head><title>Voice Playback</title></head>
        <body style="background:#0f0f0f;color:#e0e0e0;font-family:sans-serif;display:flex;justify-content:center;align-items:center;height:100vh;margin:0">
        <div style="text-align:center"><h2>Voice: {job_id}</h2>
        <audio controls autoplay src="{output_url}"></audio></div></body></html>"#);

    let html_path = std::env::temp_dir().join(format!("voice_{job_id}.html"));
    tokio::fs::write(&html_path, html).await?;

    // Open in default browser
    open::that(&html_path)?;

    Ok(ToolResult::success(format!(
        "Playing audio in browser. Job: {job_id}\nURL: {output_url}"
    )))
}
```

### Dependencies to add (if Option A)
```toml
[dependencies]
open = "5"  # Cross-platform "open file with default app"
```

### Dependencies to add (if Option B — native playback)
```toml
[dependencies]
rodio = "0.19"  # Audio playback
```

## Recommended Approach
**Option A** — it's the most universal, works on all platforms including headless servers with a browser available, and requires minimal new dependencies. The Qwen TTS API already serves outputs via `GET /outputs/{job_id}`, so we don't even need to proxy the audio — just point the browser at the API's public URL.

## File Changes
- `src/tool/voice.rs` — add `speak_stream` action (or split into `src/tool/voice_playback.rs` to respect 50-line limit)
- `Cargo.toml` — add `open = "5"` dependency (if Option A)
