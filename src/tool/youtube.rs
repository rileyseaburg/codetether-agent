//! YouTube Tool - Publish podcast episodes and videos to YouTube.
//!
//! Connects to the Voice API's YouTube endpoints to:
//! - Publish podcast episodes as YouTube videos (auto-generates waveform video)
//! - Upload existing video files to YouTube
//! - Check YouTube channel connectivity and status

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(600); // Video generation + upload can be slow

fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}

pub struct YouTubeTool {
    client: reqwest::Client,
}

impl Default for YouTubeTool {
    fn default() -> Self {
        Self::new()
    }
}

impl YouTubeTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    async fn publish_episode(&self, params: &PublishEpisodeParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/youtube/publish-episode");

        let privacy = params
            .privacy_status
            .as_deref()
            .unwrap_or("unlisted");

        let mut form = reqwest::multipart::Form::new()
            .text("podcast_id", params.podcast_id.clone())
            .text("episode_id", params.episode_id.clone())
            .text("privacy_status", privacy.to_string());

        if let Some(ref title) = params.custom_title {
            form = form.text("custom_title", title.clone());
        }
        if let Some(ref desc) = params.custom_description {
            form = form.text("custom_description", desc.clone());
        }
        if let Some(ref tags) = params.tags {
            form = form.text("tags", tags.clone());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("YouTube publish request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "YouTube publish failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let video_id = body["video_id"].as_str().unwrap_or("unknown");
        let video_url = body["url"].as_str().unwrap_or("unknown");
        let title = body["title"].as_str().unwrap_or("unknown");
        let channel = body["channel_title"].as_str().unwrap_or("unknown");

        Ok(ToolResult::success(format!(
            "Published to YouTube!\n\
             Video: {video_url}\n\
             Title: {title}\n\
             Channel: {channel}\n\
             Video ID: {video_id}"
        ))
        .with_metadata("video_id", json!(video_id))
        .with_metadata("url", json!(video_url))
        .with_metadata("title", json!(title)))
    }

    async fn upload_video(&self, params: &UploadVideoParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/youtube/upload");

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

        let privacy = params
            .privacy_status
            .as_deref()
            .unwrap_or("unlisted");

        let mut form = reqwest::multipart::Form::new()
            .part("video", part)
            .text("title", params.title.clone())
            .text("privacy_status", privacy.to_string());

        if let Some(ref desc) = params.description {
            form = form.text("description", desc.clone());
        }
        if let Some(ref tags) = params.tags {
            form = form.text("tags", tags.clone());
        }
        if let Some(ref category) = params.category_id {
            form = form.text("category_id", category.clone());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("YouTube upload request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "YouTube upload failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let video_id = body["video_id"].as_str().unwrap_or("unknown");
        let video_url = body["url"].as_str().unwrap_or("unknown");

        Ok(ToolResult::success(format!(
            "Uploaded to YouTube!\n\
             Video: {video_url}\n\
             Video ID: {video_id}"
        ))
        .with_metadata("video_id", json!(video_id))
        .with_metadata("url", json!(video_url)))
    }

    async fn status(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/youtube/status");

        let resp = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("YouTube status check failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "YouTube status check failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let status = body["status"].as_str().unwrap_or("unknown");
        let channel_title = body["channel_title"].as_str().unwrap_or("unknown");
        let channel_id = body["channel_id"].as_str().unwrap_or("unknown");
        let subs = body["subscriber_count"].as_str().unwrap_or("0");
        let videos = body["video_count"].as_str().unwrap_or("0");

        Ok(ToolResult::success(format!(
            "YouTube API Status: {status}\n\
             Channel: {channel_title}\n\
             Channel ID: {channel_id}\n\
             Subscribers: {subs}\n\
             Videos: {videos}"
        ))
        .with_metadata("channel_id", json!(channel_id))
        .with_metadata("channel_title", json!(channel_title)))
    }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    podcast_id: Option<String>,
    #[serde(default)]
    episode_id: Option<String>,
    #[serde(default)]
    privacy_status: Option<String>,
    #[serde(default)]
    custom_title: Option<String>,
    #[serde(default)]
    custom_description: Option<String>,
    #[serde(default)]
    tags: Option<String>,
    #[serde(default)]
    file_path: Option<String>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    category_id: Option<String>,
}

#[derive(Deserialize)]
struct PublishEpisodeParams {
    podcast_id: String,
    episode_id: String,
    privacy_status: Option<String>,
    custom_title: Option<String>,
    custom_description: Option<String>,
    tags: Option<String>,
}

#[derive(Deserialize)]
struct UploadVideoParams {
    file_path: String,
    title: String,
    description: Option<String>,
    tags: Option<String>,
    privacy_status: Option<String>,
    category_id: Option<String>,
}

#[async_trait]
impl Tool for YouTubeTool {
    fn id(&self) -> &str {
        "youtube"
    }
    fn name(&self) -> &str {
        "YouTube"
    }
    fn description(&self) -> &str {
        "Publish content to YouTube. Actions: publish_episode (convert podcast episode to video and upload), \
         upload_video (upload an existing video file), status (check YouTube channel connectivity). \
         Uses the Voice API's YouTube integration with OAuth2 authentication."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["publish_episode", "upload_video", "status"],
                    "description": "Action to perform"
                },
                "podcast_id": {
                    "type": "string",
                    "description": "Podcast ID (required for publish_episode)"
                },
                "episode_id": {
                    "type": "string",
                    "description": "Episode ID (required for publish_episode)"
                },
                "privacy_status": {
                    "type": "string",
                    "enum": ["public", "unlisted", "private"],
                    "description": "Video privacy (default: unlisted)",
                    "default": "unlisted"
                },
                "custom_title": {
                    "type": "string",
                    "description": "Override the episode title for the YouTube video"
                },
                "custom_description": {
                    "type": "string",
                    "description": "Override the episode description for YouTube"
                },
                "tags": {
                    "type": "string",
                    "description": "Comma-separated tags for the YouTube video"
                },
                "file_path": {
                    "type": "string",
                    "description": "Path to video file (required for upload_video)"
                },
                "title": {
                    "type": "string",
                    "description": "Video title (required for upload_video)"
                },
                "description": {
                    "type": "string",
                    "description": "Video description (for upload_video)"
                },
                "category_id": {
                    "type": "string",
                    "description": "YouTube category ID (default: 28 = Science & Technology)",
                    "default": "28"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "publish_episode" => {
                let podcast_id = match p.podcast_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "youtube",
                        "'podcast_id' is required for publish_episode",
                        Some(vec!["podcast_id"]),
                        Some(json!({"action": "publish_episode", "podcast_id": "abc123", "episode_id": "xyz789"})),
                    )),
                };
                let episode_id = match p.episode_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "youtube",
                        "'episode_id' is required for publish_episode",
                        Some(vec!["episode_id"]),
                        Some(json!({"action": "publish_episode", "podcast_id": "abc123", "episode_id": "xyz789"})),
                    )),
                };
                self.publish_episode(&PublishEpisodeParams {
                    podcast_id,
                    episode_id,
                    privacy_status: p.privacy_status,
                    custom_title: p.custom_title,
                    custom_description: p.custom_description,
                    tags: p.tags,
                }).await
            }
            "upload_video" => {
                let file_path = match p.file_path {
                    Some(f) if !f.trim().is_empty() => f,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "youtube",
                        "'file_path' is required for upload_video",
                        Some(vec!["file_path"]),
                        Some(json!({"action": "upload_video", "file_path": "/path/to/video.mp4", "title": "My Video"})),
                    )),
                };
                let title = match p.title {
                    Some(t) if !t.trim().is_empty() => t,
                    _ => return Ok(ToolResult::structured_error(
                        "MISSING_PARAM", "youtube",
                        "'title' is required for upload_video",
                        Some(vec!["title"]),
                        Some(json!({"action": "upload_video", "file_path": "/path/to/video.mp4", "title": "My Video"})),
                    )),
                };
                self.upload_video(&UploadVideoParams {
                    file_path,
                    title,
                    description: p.description,
                    tags: p.tags,
                    privacy_status: p.privacy_status,
                    category_id: p.category_id,
                }).await
            }
            "status" => self.status().await,
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION", "youtube",
                &format!("Unknown action '{other}'. Use: publish_episode, upload_video, status"),
                None,
                Some(json!({"action": "status"})),
            )),
        }
    }
}
