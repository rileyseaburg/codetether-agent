//! Voice Tool - Text-to-speech, transcription, and voice cloning via Qwen TTS API.
//!
//! Connects to the Voice Cloning Service (Qwen3-TTS) for:
//! - Speaking text with a saved voice profile
//! - Transcribing audio files to text
//! - Listing available voice profiles

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

fn default_voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}

pub struct VoiceTool {
    client: reqwest::Client,
}

impl Default for VoiceTool {
    fn default() -> Self {
        Self::new()
    }
}

impl VoiceTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    async fn speak(&self, params: &SpeakParams) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let voice_id = params
            .voice_id
            .as_deref()
            .unwrap_or("960f89fc");

        let url = format!("{base_url}/voices/{voice_id}/speak");

        let lang = params
            .language
            .clone()
            .unwrap_or_else(|| "english".into());

        let form = reqwest::multipart::Form::new()
            .text("text", params.text.clone())
            .text("language", lang);

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Voice API request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Voice API returned {status}: {body}"
            )));
        }

        let job_id = resp
            .headers()
            .get("x-job-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        // Save the WAV to a temp file
        let bytes = resp.bytes().await.context("Failed to read audio response")?;
        let output_dir = std::env::current_dir().unwrap_or_else(|_| ".".into());
        let output_path = output_dir.join(format!("voice_{job_id}.wav"));
        tokio::fs::write(&output_path, &bytes)
            .await
            .context("Failed to save audio file")?;

        let duration_secs = bytes.len() as f64 / (24000.0 * 2.0); // 24kHz, 16-bit mono

        Ok(ToolResult::success(format!(
            "Generated speech saved to: {}\nJob ID: {job_id}\nApprox duration: {duration_secs:.1}s\nSize: {} bytes",
            output_path.display(),
            bytes.len()
        ))
        .with_metadata("job_id", json!(job_id))
        .with_metadata("output_path", json!(output_path.to_string_lossy()))
        .with_metadata("size_bytes", json!(bytes.len())))
    }

    async fn transcribe(&self, params: &TranscribeParams) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let url = format!("{base_url}/transcribe");

        let file_path = std::path::Path::new(&params.file_path);
        if !file_path.exists() {
            return Ok(ToolResult::error(format!(
                "File not found: {}",
                params.file_path
            )));
        }

        let file_bytes = tokio::fs::read(file_path)
            .await
            .context("Failed to read audio file")?;

        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")?;

        let form = reqwest::multipart::Form::new().part("audio_file", part);

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Transcription request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Transcription API returned {status}: {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let text = body["transcription"]
            .as_str()
            .unwrap_or("(no transcription returned)");

        Ok(ToolResult::success(text).with_metadata("file", json!(params.file_path)))
    }

    async fn list_voices(&self) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let url = format!("{base_url}/voices");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Voice API request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Voice API returned {status}: {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let voices = body["voices"].as_array();

        match voices {
            Some(voices) if !voices.is_empty() => {
                let mut output = String::from("Available voices:\n\n");
                for v in voices {
                    let id = v["voice_id"].as_str().unwrap_or("?");
                    let name = v["name"].as_str().unwrap_or("?");
                    let dur = v["duration_seconds"].as_f64().unwrap_or(0.0);
                    let created = v["created_at"].as_str().unwrap_or("?");
                    output.push_str(&format!(
                        "- {name} (id: {id}, sample: {dur:.1}s, created: {created})\n"
                    ));
                }
                Ok(ToolResult::success(output).with_metadata("count", json!(voices.len())))
            }
            _ => Ok(ToolResult::success("No voices found. Create one by uploading a voice sample.")),
        }
    }

    async fn health(&self) -> Result<ToolResult> {
        let base_url = default_voice_api_url();
        let url = format!("{base_url}/health");

        let resp = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Voice API health check failed: {e}"))?;

        let body: Value = resp.json().await.context("Failed to parse health response")?;
        let status = body["status"].as_str().unwrap_or("unknown");
        let tts_loaded = body["tts_model_loaded"].as_bool().unwrap_or(false);
        let whisper_loaded = body["whisper_model_loaded"].as_bool().unwrap_or(false);

        Ok(ToolResult::success(format!(
            "Voice API Status: {status}\nTTS model: {}\nWhisper model: {}",
            if tts_loaded { "loaded" } else { "not loaded" },
            if whisper_loaded { "loaded" } else { "not loaded" },
        )))
    }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    voice_id: Option<String>,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    file_path: Option<String>,
}

#[derive(Deserialize)]
struct SpeakParams {
    text: String,
    voice_id: Option<String>,
    language: Option<String>,
}

#[derive(Deserialize)]
struct TranscribeParams {
    file_path: String,
}

#[async_trait]
impl Tool for VoiceTool {
    fn id(&self) -> &str {
        "voice"
    }
    fn name(&self) -> &str {
        "Voice"
    }
    fn description(&self) -> &str {
        "Text-to-speech, transcription, and voice management via Qwen TTS. Actions: speak (text to speech with cloned voice), transcribe (audio file to text), list_voices (show available voice profiles), health (check API status). Set CODETETHER_VOICE_API_URL to override the default endpoint."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["speak", "transcribe", "list_voices", "health"],
                    "description": "Action to perform"
                },
                "text": {
                    "type": "string",
                    "description": "Text to speak (required for 'speak' action)"
                },
                "voice_id": {
                    "type": "string",
                    "description": "Voice profile ID (optional for 'speak', defaults to Riley voice)"
                },
                "language": {
                    "type": "string",
                    "description": "Language for speech (default: english)",
                    "default": "english"
                },
                "file_path": {
                    "type": "string",
                    "description": "Path to audio/video file (required for 'transcribe' action)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "speak" => {
                let text = match p.text {
                    Some(t) if !t.trim().is_empty() => t,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "voice",
                            "The 'text' parameter is required for the 'speak' action",
                            Some(vec!["text"]),
                            Some(json!({"action": "speak", "text": "Hello world"})),
                        ))
                    }
                };
                self.speak(&SpeakParams {
                    text,
                    voice_id: p.voice_id,
                    language: p.language,
                })
                .await
            }
            "transcribe" => {
                let file_path = match p.file_path {
                    Some(f) if !f.trim().is_empty() => f,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "voice",
                            "The 'file_path' parameter is required for the 'transcribe' action",
                            Some(vec!["file_path"]),
                            Some(json!({"action": "transcribe", "file_path": "/path/to/audio.wav"})),
                        ))
                    }
                };
                self.transcribe(&TranscribeParams { file_path }).await
            }
            "list_voices" => self.list_voices().await,
            "health" => self.health().await,
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "voice",
                &format!("Unknown action '{other}'. Use: speak, transcribe, list_voices, health"),
                None,
                Some(json!({"action": "speak", "text": "Hello world"})),
            )),
        }
    }
}
