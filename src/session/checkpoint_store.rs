//! Sidecar persistence for resumable run checkpoints.

use super::checkpoint::RunCheckpoint;
use super::types::Session;
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

impl Session {
    pub async fn save_run_checkpoint(&mut self, checkpoint: RunCheckpoint) -> Result<PathBuf> {
        let path = Self::checkpoint_path(&self.id)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        self.metadata.run_checkpoint = Some(checkpoint.clone());
        fs::write(&path, serde_json::to_vec_pretty(&checkpoint)?).await?;
        self.save().await?;
        Ok(path)
    }

    pub async fn load_run_checkpoint(&self) -> Result<Option<RunCheckpoint>> {
        if let Some(checkpoint) = &self.metadata.run_checkpoint {
            return Ok(Some(checkpoint.clone()));
        }
        let path = Self::checkpoint_path(&self.id)?;
        match fs::read_to_string(path).await {
            Ok(raw) => Ok(Some(serde_json::from_str(&raw)?)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn clear_run_checkpoint(&mut self) -> Result<()> {
        self.metadata.run_checkpoint = None;
        let path = Self::checkpoint_path(&self.id)?;
        match fs::remove_file(path).await {
            Ok(()) => {}
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
            Err(err) => return Err(err.into()),
        }
        self.save().await
    }

    fn checkpoint_path(id: &str) -> Result<PathBuf> {
        if id.is_empty()
            || id.len() > 128
            || id.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
        {
            anyhow::bail!("Invalid session ID: rejecting checkpoint path traversal risk");
        }
        Ok(Self::sessions_dir()?.join(format!("{id}.checkpoint.json")))
    }
}
