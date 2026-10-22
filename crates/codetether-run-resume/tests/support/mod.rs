use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{ContentPart, Message, Role};
use codetether_run_checkpoint::{RunCheckpoint, storage::Storage};
use codetether_run_resume::RunSession;
use codetether_session_events::{SessionEvent, SessionResult};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};

pub struct Session {
    id: String,
    pub window: Vec<Message>,
    pub full: Vec<Message>,
    pub checkpoint: Option<RunCheckpoint>,
    pub attempts: Vec<String>,
    pub cleared: bool,
}
impl Session {
    pub fn new(id: &str) -> Self {
        let source = Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "Do not deploy".into(),
            }],
        };
        Self {
            id: id.into(),
            window: vec![source.clone(); 2],
            full: vec![source; 20],
            checkpoint: None,
            attempts: vec![],
            cleared: false,
        }
    }
    fn attempt(&mut self, prompt: &str) -> SessionResult {
        self.attempts.push(prompt.into());
        self.full.push(Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "investigating".into(),
            }],
        });
        self.window = self.full[self.full.len() - 2..].to_vec();
        codetether_step_limit::begin(Some(1));
        SessionResult {
            text: "budget exhausted".into(),
            session_id: self.id.clone(),
        }
    }
}
#[async_trait]
impl Storage for Session {
    fn checkpoint_cleared(&self) -> bool {
        self.cleared
    }
    fn set_checkpoint_cleared(&mut self, cleared: bool) {
        self.cleared = cleared;
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn checkpoint(&self) -> Option<&RunCheckpoint> {
        self.checkpoint.as_ref()
    }
    fn set_checkpoint(&mut self, cp: Option<RunCheckpoint>) {
        self.checkpoint = cp;
    }
    fn sessions_dir() -> Result<PathBuf> {
        static DIR: OnceLock<tempfile::TempDir> = OnceLock::new();
        Ok(DIR
            .get_or_init(|| tempfile::tempdir().unwrap())
            .path()
            .into())
    }
    async fn save_session(&self) -> Result<()> {
        Ok(())
    }
}
#[async_trait]
impl RunSession for Session {
    type Registry = ();
    fn messages(&self) -> &[Message] {
        &self.window
    }
    fn message_count(&self) -> usize {
        self.full.len()
    }
    async fn build_checkpoint(
        &self,
        objective: &str,
        steps: usize,
        workspace: &Path,
    ) -> Result<RunCheckpoint> {
        Ok(RunCheckpoint::from_session_messages(
            objective,
            steps,
            &self.id,
            Some(workspace.into()),
            self.full.len(),
            &self.full,
        ))
    }
    fn set_max_steps(&mut self, _: Option<usize>) {}
    fn default_max_steps() -> usize {
        1
    }
    async fn load_registry() -> Result<Arc<()>> {
        Ok(Arc::new(()))
    }
    async fn prompt(&mut self, prompt: &str) -> Result<SessionResult> {
        Ok(self.attempt(prompt))
    }
    async fn prompt_with_events(
        &mut self,
        prompt: &str,
        _: tokio::sync::mpsc::Sender<SessionEvent<Self>>,
        _: Arc<()>,
    ) -> Result<SessionResult> {
        Ok(self.attempt(prompt))
    }
}
