# Feature Request: Voice Input — Talk to CodeTether

## Requested By
Spotless Bin Co project — Riley

## Summary
Add voice-to-text input so users can speak to CodeTether instead of typing.
Press a key in the TUI, record from microphone, transcribe via Whisper on the
Voice API, inject the transcribed text as the user's input.

## Architecture

```
User speaks into Microphone, cpal captures at 16kHz mono, hound encodes to WAV,
POST goes to Voice API /transcribe endpoint, Whisper returns text, text injected
into TUI input field.
```

## TUI UX Flow

1. User presses `Ctrl+R` in chat mode
2. Status bar shows `Recording... (Ctrl+R to stop)` in red
3. Audio captures from default microphone at 16kHz mono
4. User presses `Ctrl+R` again to stop (auto-stop after 60s)
5. Status changes to `Transcribing...`
6. WAV is POSTed to Voice API `/transcribe`
7. Transcribed text is inserted into the input field
8. User can edit the text, then press Enter to submit

## CLI Usage

```bash
codetether voice-input                    # Record, transcribe, print to stdout
codetether voice-input --submit           # Record, transcribe, feed to agent
codetether voice-input --max-duration 30  # Auto-stop after 30 seconds
```

## API Reference

The Voice API at `CODETETHER_VOICE_API_URL` (default `https://voice.quantum-forge.io`):

`POST /transcribe` with multipart field `audio_file` returns:
```json
{"transcription": "Hello, what I want to do is..."}
```

Whisper loads on-demand (first call ~5s on GPU).

## New Dependencies

```toml
cpal = "0.15"     # Cross-platform audio capture
hound = "3.5"     # WAV encoding
```

## New Files

| File | Responsibility |
|------|---------------|
| `src/tool/voice_input.rs` | Tool struct, Tool trait, dispatch |
| `src/tool/voice_input/recorder.rs` | cpal mic capture, returns Vec of i16 samples |
| `src/tool/voice_input/encoder.rs` | hound WAV encoding from i16 samples |
| `src/tool/voice_input/schema.rs` | JSON schema for tool params |
| `src/tool/voice_input/actions.rs` | record_then_transcribe action |
| `src/tui/app/event_handlers/voice.rs` | Ctrl+R keybinding handler for TUI |

## Key Design Decisions

1. **Why cpal over rodio?** cpal is the low-level cross-platform audio crate.
   rodio builds on top but adds playback deps we don't need. We only need capture.

2. **Why not Web Speech API?** The user talks to CodeTether via terminal, not
   browser. The TUI (crossterm + ratatui) is the primary interface.

3. **Why not local Whisper?** The Voice API already has Whisper on GPU. No need
   to bundle a 1GB model. The HTTP call adds ~1-2s latency.

4. **Recording format** — 16kHz mono 16-bit PCM WAV (what Whisper expects).

5. **Max duration** — Default 60 seconds, configurable via env
   CODETETHER_VOICE_INPUT_MAX_SECS. Prevents runaway recording.

6. **Error handling** — No microphone found: show "No audio input device found.
   Connect a microphone and try again." API down: show "Voice API unavailable.
   Check CODETETHER_VOICE_API_URL."

## Recorder Implementation

```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

pub fn record(max_duration_secs: u64, stop_flag: Arc<AtomicBool>) -> Result<Vec<i16>> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No audio input device found"))?;
    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(16000),
        buffer_size: cpal::BufferSize::Default,
    };
    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    let sc = samples.clone();
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buf = sc.lock().unwrap();
            for &s in data { buf.push((s * 32767.0).clamp(-32768.0, 32767.0) as i16); }
        },
        |err| tracing::error!("Audio capture error: {err}"),
        None,
    )?;
    stream.play()?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(max_duration_secs);
    while !stop_flag.load(Ordering::Relaxed) && std::time::Instant::now() < deadline {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    drop(stream);
    Ok(Arc::try_unwrap(samples).unwrap().into_inner().unwrap())
}
```

## WAV Encoder

```rust
pub fn encode_wav(samples: &[i16]) -> Result<Vec<u8>> {
    let spec = hound::WavSpec {
        channels: 1, sample_rate: 16000,
        bits_per_sample: 16, sample_format: hound::SampleFormat::Int,
    };
    let mut buf = Vec::new();
    let mut writer = hound::WavWriter::new(&mut buf, spec)?;
    for &s in samples { writer.write_sample(s)?; }
    writer.finalize()?;
    Ok(buf)
}
```

## Constraints
- No local Whisper — always use the remote API
- GPU not required on client — recording is CPU-only, transcription is server-side
- Must work on Linux — cpal supports ALSA/PulseAudio
- Graceful degradation — if cpal fails (no mic), show clear error, don't crash
- 50-line file limit — split into separate files
- Never `cargo build` or `cargo check` — let CI catch errors
