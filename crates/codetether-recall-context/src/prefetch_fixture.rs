//! In-memory projections for two conversations sharing one real workspace.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_recall_index::{IndexedSession, ProjectionInput, project};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Mutex,
};

pub(super) struct Fixture {
    pub dir: tempfile::TempDir,
    pub projections: HashMap<String, serde_json::Value>,
    pub reads: Mutex<Vec<String>>,
}

pub(super) struct Session {
    pub id: String,
    pub workspace: PathBuf,
    pub messages: Vec<Message>,
    pub allowed: bool,
}

impl crate::Source for Session {
    fn prior_context_allowed(&self) -> bool {
        self.allowed
    }
    fn workspace(&self) -> Option<&Path> {
        Some(&self.workspace)
    }
    fn messages(&self) -> &[Message] {
        &self.messages
    }
    fn id(&self) -> &str {
        &self.id
    }
}

impl Fixture {
    pub fn new() -> Self {
        let mut fixture = Self {
            dir: tempfile::tempdir().unwrap(),
            projections: HashMap::new(),
            reads: Mutex::default(),
        };
        for id in ["quote-session", "ads-session"] {
            let messages = vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!("quote template include {id}"),
                }],
            }];
            let index = project(ProjectionInput {
                messages: &messages,
                session_id: id,
                title: None,
                workspace: fixture.dir.path().canonicalize().unwrap(),
                updated_at: "2026-01-01T00:00:00Z".parse().unwrap(),
                message_offset: 0,
                message_count: 1,
            });
            fixture
                .projections
                .insert(id.into(), serde_json::to_value(index).unwrap());
        }
        fixture
    }
    pub fn session(&self, id: &str) -> Session {
        Session {
            id: id.into(),
            workspace: self.dir.path().into(),
            allowed: true,
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "quote template include".into(),
                }],
            }],
        }
    }
}

#[async_trait]
impl codetether_recall_store::Host for Fixture {
    fn session_path(&self, id: &str) -> Result<PathBuf> {
        Ok(self.dir.path().join(format!("{id}.json")))
    }
    fn root(&self) -> Result<PathBuf> {
        Ok(self.dir.path().join("recall"))
    }
    async fn projection(&self, id: &str) -> Result<Option<serde_json::Value>> {
        self.reads.lock().unwrap().push(id.into());
        Ok(self.projections.get(id).cloned())
    }
}

#[async_trait]
impl codetether_recall_query::Host for Fixture {
    type Session = ();
    fn schedule_backfill(&self, _: &Path) {}
    async fn load_tail(&self, _: &str, _: usize) -> Result<()> {
        anyhow::bail!("no transcript fixture")
    }
    fn build(_: &()) -> Option<IndexedSession> {
        None
    }
}
