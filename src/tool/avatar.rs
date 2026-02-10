//! Avatar Tool - AI digital human video generation via Duix Avatar.
//!
//! Connects to the Duix Avatar integration on the Voice API to:
//! - Generate lip-synced avatar videos from audio or text
//! - Manage avatar models (upload, list)
//! - Control the Duix Avatar container (start/stop for GPU sharing)
//! - Create avatar videos from podcast episodes with optional YouTube upload

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(900); // Avatar video gen can be slow

fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}

pub struct AvatarTool {
    client: reqwest::Client,
}

impl Default for AvatarTool {
    fn default() -> Self {
        Self::new()
    }
}

impl AvatarTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    async fn status(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/status");

        let resp = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar status check failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Avatar status failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let running = body["container_running"].as_bool().unwrap_or(false);
        let image_size = body["image_size"].as_str().unwrap_or("unknown");
        let models = body["models"].as_array().map(|m| m.len()).unwrap_or(0);
        let gpu_sharing = body["gpu_sharing_enabled"].as_bool().unwrap_or(false);

        Ok(ToolResult::success(format!(
            "Duix Avatar Status:\n\
             Container: {}\n\
             Docker Image: {image_size}\n\
             Models available: {models}\n\
             GPU time-sharing: {}",
            if running { "running" } else { "stopped" },
            if gpu_sharing { "enabled" } else { "disabled" },
        ))
        .with_metadata("container_running", json!(running))
        .with_metadata("models_count", json!(models)))
    }

    async fn start(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/start");

        let resp = self
            .client
            .post(&url)
            .timeout(Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar start failed: {e}"))?;

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let status = body["status"].as_str().unwrap_or("unknown");
        let message = body["message"].as_str().unwrap_or("");

        if status == "started" {
            Ok(ToolResult::success(format!(
                "Duix Avatar container started. {message}"
            )))
        } else {
            Ok(ToolResult::error(format!(
                "Failed to start avatar container: {message}"
            )))
        }
    }

    async fn stop(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/stop");

        let resp = self
            .client
            .post(&url)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar stop failed: {e}"))?;

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let message = body["message"].as_str().unwrap_or("Container stopped");

        Ok(ToolResult::success(message))
    }

    async fn generate(&self, params: &GenerateParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/generate");

        let mut form = reqwest::multipart::Form::new();

        if let Some(ref text) = params.text {
            form = form.text("text", text.clone());
        }
        if let Some(ref audio_url) = params.audio_url {
            form = form.text("audio_url", audio_url.clone());
        }
        if let Some(ref audio_file) = params.audio_file {
            let file_path = std::path::Path::new(audio_file);
            if !file_path.exists() {
                return Ok(ToolResult::error(format!(
                    "Audio file not found: {audio_file}"
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
                .mime_str("audio/wav")?;
            form = form.part("audio_file", part);
        }
        if let Some(ref model) = params.model_video {
            form = form.text("model_video", model.clone());
        }
        if let Some(ref voice_id) = params.voice_id {
            form = form.text("voice_id", voice_id.clone());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar generate request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Avatar generation failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;

        if let Some(error) = body["error"].as_str() {
            return Ok(ToolResult::error(format!("Avatar generation error: {error}")));
        }

        let video_path = body["video_path"].as_str().unwrap_or("unknown");
        let job_code = body["job_code"].as_str().unwrap_or("unknown");

        Ok(ToolResult::success(format!(
            "Avatar video generated!\n\
             Video: {video_path}\n\
             Job: {job_code}"
        ))
        .with_metadata("video_path", json!(video_path))
        .with_metadata("job_code", json!(job_code)))
    }

    async fn generate_from_episode(&self, params: &EpisodeAvatarParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/generate-from-episode");

        let mut form = reqwest::multipart::Form::new()
            .text("podcast_id", params.podcast_id.clone())
            .text("episode_id", params.episode_id.clone());

        if let Some(ref model) = params.model_video {
            form = form.text("model_video", model.clone());
        }
        if params.upload_youtube {
            form = form.text("upload_youtube", "true".to_string());
        }
        if let Some(ref privacy) = params.privacy_status {
            form = form.text("privacy_status", privacy.clone());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Avatar episode generation failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Avatar episode generation failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;

        if let Some(error) = body["error"].as_str() {
            return Ok(ToolResult::error(format!("Avatar generation error: {error}")));
        }

        let video_path = body["video_path"].as_str().unwrap_or("unknown");
        let title = body["episode_title"].as_str().unwrap_or("unknown");
        let mut output = format!(
            "Avatar video generated from episode!\n\
             Title: {title}\n\
             Video: {video_path}"
        );

        if let Some(yt) = body.get("youtube") {
            let yt_url = yt["url"].as_str().unwrap_or("unknown");
            let yt_id = yt["video_id"].as_str().unwrap_or("unknown");
            output.push_str(&format!("\n\nUploaded to YouTube!\nURL: {yt_url}\nVideo ID: {yt_id}"));
        }

        if let Some(yt_err) = body["youtube_error"].as_str() {
            output.push_str(&format!("\n\nYouTube upload error: {yt_err}"));
        }

        Ok(ToolResult::success(output)
            .with_metadata("video_path", json!(video_path))
            .with_metadata("episode_title", json!(title)))
    }

    async fn list_models(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/models");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("List models failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "List models failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let models = body["models"].as_array();

        match models {
            Some(models) if !models.is_empty() => {
                let mut output = format!("Avatar models ({}):\n\n", models.len());
                for m in models {
                    let name = m["name"].as_str().unwrap_or("?");
                    let path = m["path"].as_str().unwrap_or("?");
                    let size = m["size_bytes"].as_u64().unwrap_or(0);
                    let size_mb = size as f64 / 1_048_576.0;
                    output.push_str(&format!("- {name} ({size_mb:.1}MB)\n  Path: {path}\n"));
                }
                Ok(ToolResult::success(output).with_metadata("count", json!(models.len())))
            }
            _ => Ok(ToolResult::success(
                "No avatar models found. Upload a silent video of yourself \
                 using action 'upload_model' to create an AI clone.",
            )),
        }
    }

    async fn upload_model(&self, params: &UploadModelParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/avatar/upload-model");

        let file_path = std::path::Path::new(&params.file_path);
        if !file_path.exists() {
            return Ok(ToolResult::error(format!(
                "Video file not found: {}",
                params.file_path
            )));
        }

        let file_bytes = tokio::fs::read(file_path)
            .await
            .context("Failed to read video file")?;
        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("video/mp4")?;

        let form = reqwest::multipart::Form::new()
            .text("name", params.name.clone())
            .part("video", part);

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Upload model failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Upload model failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let model_path = body["model_path"].as_str().unwrap_or("unknown");
        let size = body["size_bytes"].as_u64().unwrap_or(0);
        let size_mb = size as f64 / 1_048_576.0;

        Ok(ToolResult::success(format!(
            "Model uploaded!\n\
             Name: {}\n\
             Path: {model_path}\n\
             Size: {size_mb:.1}MB\n\n\
             You can now use this model for avatar video generation.",
            params.name
        ))
        .with_metadata("model_path", json!(model_path)))
    }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    audio_url: Option<String>,
    #[serde(default)]
    audio_file: Option<String>,
    #[serde(default)]
    model_video: Option<String>,
    #[serde(default)]
    voice_id: Option<String>,
    #[serde(default)]
    podcast_id: Option<String>,
    #[serde(default)]
    episode_id: Option<String>,
    #[serde(default)]
    upload_youtube: Option<bool>,
    #[serde(default)]
    privacy_status: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    file_path: Option<String>,
}

#[derive(Deserialize)]
struct GenerateParams {
    text: Option<String>,
    audio_url: Option<String>,
    audio_file: Option<String>,
    model_video: Option<String>,
    voice_id: Option<String>,
}

#[derive(Deserialize)]
struct EpisodeAvatarParams {
    podcast_id: String,
    episode_id: String,
    model_video: Option<String>,
    upload_youtube: bool,
    privacy_status: Option<String>,
}

#[derive(Deserialize)]
struct UploadModelParams {
    name: String,
    file_path: String,
}

#[async_trait]
impl Tool for AvatarTool {
    fn id(&self) -> &str {
        "avatar"
    }
    fn name(&self) -> &str {
        "Avatar"
    }
    fn description(&self) -> &str {
        "AI digital human video generation using Duix Avatar. Create lip-synced avatar videos \
         from audio or text. Actions: status (check service), start/stop (manage GPU container), \
         generate (create avatar video from audio), generate_from_episode (podcast → avatar video → YouTube), \
         list_models (show available avatar models), upload_model (upload a silent video for cloning). \
         Supports the full pipeline: text → speech → avatar video → YouTube."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["status", "start", "stop", "generate", "generate_from_episode", "list_models", "upload_model"],
                    "description": "Action to perform"
                },
                "text": {
                    "type": "string",
                    "description": "Text to convert to speech then avatar video (for 'generate')"
                },
                "audio_url": {
                    "type": "string",
                    "description": "Path to audio file on server (for 'generate')"
                },
                "audio_file": {
                    "type": "string",
                    "description": "Local path to audio file to upload (for 'generate')"
                },
                "model_video": {
                    "type": "string",
                    "description": "Path to model video for lip-sync (optional, uses default)"
                },
                "voice_id": {
                    "type": "string",
                    "description": "Voice ID for TTS (default: Riley 960f89fc)"
                },
                "podcast_id": {
                    "type": "string",
                    "description": "Podcast ID (for 'generate_from_episode')"
                },
                "episode_id": {
                    "type": "string",
                    "description": "Episode ID (for 'generate_from_episode')"
                },
                "upload_youtube": {
                    "type": "boolean",
                    "description": "Upload generated video to YouTube (for 'generate_from_episode')",
                    "default": false
                },
                "privacy_status": {
                    "type": "string",
                    "enum": ["public", "unlisted", "private"],
                    "description": "YouTube privacy status (default: unlisted)"
                },
                "name": {
                    "type": "string",
                    "description": "Model name (for 'upload_model')"
                },
                "file_path": {
                    "type": "string",
                    "description": "Path to video file (for 'upload_model')"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "status" => self.status().await,
            "start" => self.start().await,
            "stop" => self.stop().await,
            "generate" => {
                self.generate(&GenerateParams {
                    text: p.text,
                    audio_url: p.audio_url,
                    audio_file: p.audio_file,
                    model_video: p.model_video,
                    voice_id: p.voice_id,
                }).await
            }
            "generate_from_episode" => {
                let podcast_id = match p.podcast_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "avatar",
                        "'podcast_id' is required for generate_from_episode",
                        Some(vec!["podcast_id"]),
                        Some(json!({"action": "generate_from_episode", "podcast_id": "abc123", "episode_id": "xyz789"})),
                    )),
                };
                let episode_id = match p.episode_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "avatar",
                        "'episode_id' is required for generate_from_episode",
                        Some(vec!["episode_id"]),
                        Some(json!({"action": "generate_from_episode", "podcast_id": "abc123", "episode_id": "xyz789"})),
                    )),
                };
                self.generate_from_episode(&EpisodeAvatarParams {
                    podcast_id,
                    episode_id,
                    model_video: p.model_video,
                    upload_youtube: p.upload_youtube.unwrap_or(false),
                    privacy_status: p.privacy_status,
                }).await
            }
            "list_models" => self.list_models().await,
            "upload_model" => {
                let name = match p.name {
                    Some(n) if !n.trim().is_empty() => n,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "avatar",
                        "'name' is required for upload_model",
                        Some(vec!["name"]),
                        Some(json!({"action": "upload_model", "name": "Riley", "file_path": "/path/to/video.mp4"})),
                    )),
                };
                let file_path = match p.file_path {
                    Some(f) if !f.trim().is_empty() => f,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "avatar",
                        "'file_path' is required for upload_model",
                        Some(vec!["file_path"]),
                        Some(json!({"action": "upload_model", "name": "Riley", "file_path": "/path/to/video.mp4"})),
                    )),
                };
                self.upload_model(&UploadModelParams { name, file_path }).await
            }
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION", "avatar",
                &format!("Unknown action '{other}'. Use: status, start, stop, generate, generate_from_episode, list_models, upload_model"),
                None,
                Some(json!({"action": "status"})),
            )),
        }
    }
}
