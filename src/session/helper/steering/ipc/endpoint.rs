//! Active session endpoint lifecycle.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use tokio::net::UnixListener;

pub(in crate::session::helper::steering) struct Endpoint {
    path: PathBuf,
    handle: tokio::task::JoinHandle<()>,
}

impl Endpoint {
    pub(in crate::session::helper::steering) fn start(session_id: &str) -> Result<Self> {
        Self::start_at(session_id, super::path::for_session(session_id)?)
    }

    pub(super) fn start_at(session_id: &str, path: PathBuf) -> Result<Self> {
        clear_stale(&path)?;
        let listener = UnixListener::bind(&path)
            .with_context(|| format!("bind session endpoint {}", path.display()))?;
        let handle = tokio::spawn(super::server::run(listener, session_id.to_string()));
        Ok(Self { path, handle })
    }
}

fn clear_stale(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    if std::os::unix::net::UnixStream::connect(path).is_ok() {
        bail!("session already has an active owner");
    }
    std::fs::remove_file(path)?;
    Ok(())
}

impl Drop for Endpoint {
    fn drop(&mut self) {
        self.handle.abort();
        let _ = std::fs::remove_file(&self.path);
    }
}
