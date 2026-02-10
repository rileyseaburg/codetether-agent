//! Podcast Tool - Create and manage AI-generated podcasts via Qwen TTS.
//!
//! Lets the agent programmatically create podcasts, generate TTS episodes
//! from scripts using cloned voices, manage feeds, and produce RSS-ready
//! podcast content — all from natural language instructions.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::Duration;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(300); // Episodes can take a while

fn voice_api_url() -> String {
    std::env::var("CODETETHER_VOICE_API_URL")
        .unwrap_or_else(|_| "https://voice.quantum-forge.io".to_string())
}

pub struct PodcastTool {
    client: reqwest::Client,
}

impl Default for PodcastTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PodcastTool {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .user_agent("CodeTether-Agent/1.0")
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    async fn create_podcast(&self, params: &CreatePodcastParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts");

        let mut form = reqwest::multipart::Form::new()
            .text("title", params.title.clone())
            .text("description", params.description.clone())
            .text(
                "author",
                params
                    .author
                    .clone()
                    .unwrap_or_else(|| "CodeTether Agent".into()),
            );

        if let Some(ref email) = params.email {
            form = form.text("email", email.clone());
        }
        if let Some(ref category) = params.category {
            form = form.text("category", category.clone());
        }
        if let Some(ref subcategory) = params.subcategory {
            form = form.text("subcategory", subcategory.clone());
        }
        if let Some(ref language) = params.language {
            form = form.text("language", language.clone());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Create podcast request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Create podcast failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let podcast_id = body["podcast_id"].as_str().unwrap_or("unknown");
        let feed_url = body["feed_url"].as_str().unwrap_or("unknown");

        Ok(ToolResult::success(format!(
            "Podcast created!\n\
             ID: {podcast_id}\n\
             Title: {}\n\
             Feed URL: {feed_url}\n\
             \n\
             Next: Add episodes with action 'create_episode' using this podcast_id.",
            params.title
        ))
        .with_metadata("podcast_id", json!(podcast_id))
        .with_metadata("feed_url", json!(feed_url)))
    }

    async fn create_episode(&self, params: &CreateEpisodeParams) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{}/episodes", params.podcast_id);

        let voice_id = params.voice_id.as_deref().unwrap_or("960f89fc");

        let mut form = reqwest::multipart::Form::new()
            .text("title", params.title.clone())
            .text("script", params.script.clone())
            .text("voice_id", voice_id.to_string());

        if let Some(ref desc) = params.description {
            form = form.text("description", desc.clone());
        }
        if let Some(num) = params.episode_number {
            form = form.text("episode_number", num.to_string());
        }
        if let Some(season) = params.season_number {
            form = form.text("season_number", season.to_string());
        }

        let resp = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Create episode request failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Create episode failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let episode_id = body["episode_id"].as_str().unwrap_or("unknown");
        let audio_url = body["audio_url"].as_str().unwrap_or("unknown");
        let duration = body["duration"].as_str().unwrap_or("unknown");
        let feed_url = body["feed_url"].as_str().unwrap_or("unknown");

        Ok(ToolResult::success(format!(
            "Episode created!\n\
             Episode ID: {episode_id}\n\
             Title: {}\n\
             Duration: {duration}\n\
             Audio: {audio_url}\n\
             Feed: {feed_url}\n\
             \n\
             The RSS feed has been updated automatically.",
            params.title
        ))
        .with_metadata("episode_id", json!(episode_id))
        .with_metadata("audio_url", json!(audio_url))
        .with_metadata("duration", json!(duration)))
    }

    async fn list_podcasts(&self) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("List podcasts failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "List podcasts failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let podcasts = body["podcasts"].as_array().or_else(|| body.as_array());

        match podcasts {
            Some(podcasts) if !podcasts.is_empty() => {
                let mut output = format!("Found {} podcast(s):\n\n", podcasts.len());
                for p in podcasts {
                    let id = p["podcast_id"].as_str().unwrap_or("?");
                    let title = p["title"].as_str().unwrap_or("Untitled");
                    let eps = p["episode_count"].as_u64().unwrap_or(0);
                    let feed = p["feed_url"].as_str().unwrap_or("?");
                    output.push_str(&format!(
                        "- **{title}** (id: {id}, {eps} episodes)\n  Feed: {feed}\n"
                    ));
                }
                Ok(ToolResult::success(output).with_metadata("count", json!(podcasts.len())))
            }
            _ => Ok(ToolResult::success(
                "No podcasts found. Create one with action 'create_podcast'.",
            )),
        }
    }

    async fn get_podcast(&self, podcast_id: &str) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{podcast_id}");

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Get podcast failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Get podcast failed ({status}): {body}"
            )));
        }

        let body: Value = resp.json().await.context("Failed to parse response")?;
        let formatted = serde_json::to_string_pretty(&body).unwrap_or_else(|_| body.to_string());

        Ok(ToolResult::success(formatted))
    }

    async fn delete_podcast(&self, podcast_id: &str) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{podcast_id}");

        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Delete podcast failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Delete podcast failed ({status}): {body}"
            )));
        }

        Ok(ToolResult::success(format!(
            "Podcast {podcast_id} and all its episodes have been deleted."
        )))
    }

    async fn delete_episode(&self, podcast_id: &str, episode_id: &str) -> Result<ToolResult> {
        let base = voice_api_url();
        let url = format!("{base}/podcasts/{podcast_id}/episodes/{episode_id}");

        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("Delete episode failed: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Ok(ToolResult::error(format!(
                "Delete episode failed ({status}): {body}"
            )));
        }

        Ok(ToolResult::success(format!(
            "Episode {episode_id} deleted from podcast {podcast_id}. RSS feed updated."
        )))
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
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    subcategory: Option<String>,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    script: Option<String>,
    #[serde(default)]
    voice_id: Option<String>,
    #[serde(default)]
    episode_number: Option<i64>,
    #[serde(default)]
    season_number: Option<i64>,
}

#[derive(Deserialize)]
struct CreatePodcastParams {
    title: String,
    description: String,
    author: Option<String>,
    email: Option<String>,
    category: Option<String>,
    subcategory: Option<String>,
    language: Option<String>,
}

#[derive(Deserialize)]
struct CreateEpisodeParams {
    podcast_id: String,
    title: String,
    script: String,
    voice_id: Option<String>,
    description: Option<String>,
    episode_number: Option<i64>,
    season_number: Option<i64>,
}

#[async_trait]
impl Tool for PodcastTool {
    fn id(&self) -> &str {
        "podcast"
    }
    fn name(&self) -> &str {
        "Podcast"
    }
    fn description(&self) -> &str {
        "Create and manage AI-generated podcasts with cloned voice narration. \
         Actions: create_podcast (new podcast series), create_episode (generate TTS episode from script), \
         list_podcasts (show all podcasts), get_podcast (details + episodes), \
         delete_podcast, delete_episode. Episodes are auto-converted to MP3 with RSS feed generation."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["create_podcast", "create_episode", "list_podcasts", "get_podcast", "delete_podcast", "delete_episode"],
                    "description": "Action to perform"
                },
                "podcast_id": {
                    "type": "string",
                    "description": "Podcast ID (required for create_episode, get_podcast, delete_podcast, delete_episode)"
                },
                "episode_id": {
                    "type": "string",
                    "description": "Episode ID (required for delete_episode)"
                },
                "title": {
                    "type": "string",
                    "description": "Title for podcast or episode"
                },
                "description": {
                    "type": "string",
                    "description": "Description for podcast or episode"
                },
                "author": {
                    "type": "string",
                    "description": "Podcast author name (default: CodeTether Agent)"
                },
                "email": {
                    "type": "string",
                    "description": "Contact email for podcast"
                },
                "category": {
                    "type": "string",
                    "description": "Podcast category (default: Technology)"
                },
                "subcategory": {
                    "type": "string",
                    "description": "Podcast subcategory"
                },
                "language": {
                    "type": "string",
                    "description": "Podcast language code (default: en-us)"
                },
                "script": {
                    "type": "string",
                    "description": "Episode script text to convert to speech (required for create_episode)"
                },
                "voice_id": {
                    "type": "string",
                    "description": "Voice profile ID for TTS (default: Riley voice 960f89fc)"
                },
                "episode_number": {
                    "type": "integer",
                    "description": "Episode number"
                },
                "season_number": {
                    "type": "integer",
                    "description": "Season number"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "create_podcast" => {
                let title = match p.title {
                    Some(t) if !t.trim().is_empty() => t,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'title' is required for create_podcast",
                            Some(vec!["title"]),
                            Some(
                                json!({"action": "create_podcast", "title": "My Podcast", "description": "A tech podcast"}),
                            ),
                        ));
                    }
                };
                let description = match p.description {
                    Some(d) if !d.trim().is_empty() => d,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'description' is required for create_podcast",
                            Some(vec!["description"]),
                            Some(
                                json!({"action": "create_podcast", "title": "My Podcast", "description": "A tech podcast"}),
                            ),
                        ));
                    }
                };
                self.create_podcast(&CreatePodcastParams {
                    title,
                    description,
                    author: p.author,
                    email: p.email,
                    category: p.category,
                    subcategory: p.subcategory,
                    language: p.language,
                })
                .await
            }
            "create_episode" => {
                let podcast_id = match p.podcast_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'podcast_id' is required for create_episode",
                            Some(vec!["podcast_id"]),
                            Some(
                                json!({"action": "create_episode", "podcast_id": "abc12345", "title": "Episode 1", "script": "Hello listeners..."}),
                            ),
                        ));
                    }
                };
                let title = match p.title {
                    Some(t) if !t.trim().is_empty() => t,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'title' is required for create_episode",
                            Some(vec!["title"]),
                            Some(
                                json!({"action": "create_episode", "podcast_id": "abc12345", "title": "Episode 1", "script": "Hello listeners..."}),
                            ),
                        ));
                    }
                };
                let script = match p.script {
                    Some(s) if !s.trim().is_empty() => s,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'script' is required for create_episode — this is the text that will be converted to speech",
                            Some(vec!["script"]),
                            Some(
                                json!({"action": "create_episode", "podcast_id": "abc12345", "title": "Episode 1", "script": "Hello listeners..."}),
                            ),
                        ));
                    }
                };
                self.create_episode(&CreateEpisodeParams {
                    podcast_id,
                    title,
                    script,
                    voice_id: p.voice_id,
                    description: p.description,
                    episode_number: p.episode_number,
                    season_number: p.season_number,
                })
                .await
            }
            "list_podcasts" => self.list_podcasts().await,
            "get_podcast" => {
                let podcast_id = match p.podcast_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'podcast_id' is required for get_podcast",
                            Some(vec!["podcast_id"]),
                            Some(json!({"action": "get_podcast", "podcast_id": "abc12345"})),
                        ));
                    }
                };
                self.get_podcast(&podcast_id).await
            }
            "delete_podcast" => {
                let podcast_id = match p.podcast_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'podcast_id' is required for delete_podcast",
                            Some(vec!["podcast_id"]),
                            Some(json!({"action": "delete_podcast", "podcast_id": "abc12345"})),
                        ));
                    }
                };
                self.delete_podcast(&podcast_id).await
            }
            "delete_episode" => {
                let podcast_id = match p.podcast_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'podcast_id' is required for delete_episode",
                            Some(vec!["podcast_id"]),
                            Some(
                                json!({"action": "delete_episode", "podcast_id": "abc12345", "episode_id": "xyz789"}),
                            ),
                        ));
                    }
                };
                let episode_id = match p.episode_id {
                    Some(id) if !id.trim().is_empty() => id,
                    _ => {
                        return Ok(ToolResult::structured_error(
                            "MISSING_PARAM",
                            "podcast",
                            "'episode_id' is required for delete_episode",
                            Some(vec!["episode_id"]),
                            Some(
                                json!({"action": "delete_episode", "podcast_id": "abc12345", "episode_id": "xyz789"}),
                            ),
                        ));
                    }
                };
                self.delete_episode(&podcast_id, &episode_id).await
            }
            other => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "podcast",
                &format!(
                    "Unknown action '{other}'. Use: create_podcast, create_episode, list_podcasts, get_podcast, delete_podcast, delete_episode"
                ),
                None,
                Some(json!({"action": "list_podcasts"})),
            )),
        }
    }
}
