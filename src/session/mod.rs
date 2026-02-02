//! Session management
//!
//! Sessions track the conversation history and state for agent interactions.

use crate::agent::ToolUse;
use crate::provider::{Message, Usage};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

/// A conversation session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<Message>,
    pub tool_uses: Vec<ToolUse>,
    pub usage: Usage,
    pub agent: String,
    pub metadata: SessionMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub directory: Option<PathBuf>,
    pub model: Option<String>,
    pub shared: bool,
    pub share_url: Option<String>,
}

impl Session {
    /// Create a new session
    pub async fn new() -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        Ok(Self {
            id,
            title: None,
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
            tool_uses: Vec::new(),
            usage: Usage::default(),
            agent: "build".to_string(),
            metadata: SessionMetadata {
                directory: Some(std::env::current_dir()?),
                ..Default::default()
            },
        })
    }

    /// Load an existing session
    pub async fn load(id: &str) -> Result<Self> {
        let path = Self::session_path(id)?;
        let content = fs::read_to_string(&path).await?;
        let session: Session = serde_json::from_str(&content)?;
        Ok(session)
    }

    /// Load the last session
    pub async fn last() -> Result<Self> {
        let sessions_dir = Self::sessions_dir()?;
        
        if !sessions_dir.exists() {
            anyhow::bail!("No sessions found");
        }

        let mut entries: Vec<tokio::fs::DirEntry> = Vec::new();
        let mut read_dir = fs::read_dir(&sessions_dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            entries.push(entry);
        }

        if entries.is_empty() {
            anyhow::bail!("No sessions found");
        }

        // Sort by modification time (most recent first)
        // Use std::fs::metadata since we can't await in sort_by_key
        entries.sort_by_key(|e| {
            std::cmp::Reverse(
                std::fs::metadata(e.path())
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
            )
        });

        if let Some(entry) = entries.first() {
            let content: String = fs::read_to_string(entry.path()).await?;
            let session: Session = serde_json::from_str(&content)?;
            return Ok(session);
        }

        anyhow::bail!("No sessions found")
    }

    /// Save the session to disk
    pub async fn save(&self) -> Result<()> {
        let path = Self::session_path(&self.id)?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).await?;
        
        Ok(())
    }

    /// Add a message to the session
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    /// Execute a prompt and get the result
    pub async fn prompt(&self, message: &str) -> Result<SessionResult> {
        use crate::provider::{ContentPart, Role, ProviderRegistry, CompletionRequest};
        
        // Load providers from Vault
        let registry = ProviderRegistry::from_vault().await?;
        
        let providers = registry.list();
        if providers.is_empty() {
            anyhow::bail!("No providers available. Configure API keys in HashiCorp Vault.");
        }

        tracing::info!("Available providers: {:?}", providers);

        // Use first available provider (we'll make this configurable)
        let provider = registry.get(providers[0])
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", providers[0]))?;

        // Build messages
        let mut messages = self.messages.clone();
        messages.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: message.to_string() }],
        });

        // Determine model to use
        let model = self.metadata.model.clone().unwrap_or_else(|| {
            // Default models per provider
            match providers[0] {
                "moonshotai" => "kimi-k2.5".to_string(),
                "anthropic" => "claude-sonnet-4-20250514".to_string(),
                "openai" => "gpt-4o".to_string(),
                "google" => "gemini-2.5-pro".to_string(),
                _ => "kimi-k2.5".to_string(),
            }
        });

        // Kimi K2.5 requires temperature=1.0
        let temperature = if model.starts_with("kimi-k2") {
            Some(1.0)
        } else {
            Some(0.7)
        };

        tracing::info!("Using model: {} via provider: {}", model, providers[0]);

        // Create completion request
        let request = CompletionRequest {
            messages,
            tools: Vec::new(), // TODO: Add tools
            model,
            temperature,
            top_p: None,
            max_tokens: Some(4096),
            stop: Vec::new(),
        };

        // Call the provider
        let response = provider.complete(request).await?;

        // Extract text from response
        let text = response.message.content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(SessionResult {
            text,
            session_id: self.id.clone(),
        })
    }

    /// Generate a title for the session based on the first message
    pub async fn generate_title(&mut self) -> Result<()> {
        if self.title.is_some() {
            return Ok(());
        }

        // Get first user message
        let first_message = self.messages.iter().find(|m| m.role == crate::provider::Role::User);
        
        if let Some(msg) = first_message {
            let text: String = msg
                .content
                .iter()
                .filter_map(|p| match p {
                    crate::provider::ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" ");

            // Truncate to reasonable length
            self.title = Some(if text.len() > 50 {
                format!("{}...", &text[..47])
            } else {
                text
            });
        }

        Ok(())
    }

    /// Get the sessions directory
    fn sessions_dir() -> Result<PathBuf> {
        crate::config::Config::data_dir()
            .map(|d| d.join("sessions"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
    }

    /// Get the path for a session file
    fn session_path(id: &str) -> Result<PathBuf> {
        Ok(Self::sessions_dir()?.join(format!("{}.json", id)))
    }
}

/// Result from a session prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub text: String,
    pub session_id: String,
}

/// List all sessions
pub async fn list_sessions() -> Result<Vec<SessionSummary>> {
    let sessions_dir = crate::config::Config::data_dir()
        .map(|d| d.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;

    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut summaries = Vec::new();
    let mut entries = fs::read_dir(&sessions_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(&path).await {
                if let Ok(session) = serde_json::from_str::<Session>(&content) {
                    summaries.push(SessionSummary {
                        id: session.id,
                        title: session.title,
                        created_at: session.created_at,
                        updated_at: session.updated_at,
                        message_count: session.messages.len(),
                        agent: session.agent,
                    });
                }
            }
        }
    }

    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

/// Summary of a session for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub agent: String,
}

// Async helper for Vec
use futures::StreamExt;
trait AsyncCollect<T> {
    async fn collect(self) -> Vec<T>;
}

impl<S, T> AsyncCollect<T> for S
where
    S: futures::Stream<Item = T> + Unpin,
{
    async fn collect(mut self) -> Vec<T> {
        let mut items = Vec::new();
        while let Some(item) = self.next().await {
            items.push(item);
        }
        items
    }
}
