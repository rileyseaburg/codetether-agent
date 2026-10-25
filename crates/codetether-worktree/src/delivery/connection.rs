//! Open the per-workspace SQLite delivery ledger.

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::{Path, PathBuf};

const FILE: &str = "worktree-deliveries.sqlite3";

/// Locate the delivery ledger for the checkout rooted at `root`.
///
/// The ledger lives in the workspace data directory (honouring
/// `CODETETHER_DATA_DIR`), beside the session database.
///
/// # Errors
///
/// Returns an error when no data directory can be resolved.
///
/// # Examples
///
/// ```rust
/// let path = codetether_worktree::ledger_path(std::path::Path::new("/srv/project")).unwrap();
/// assert!(path.ends_with("worktree-deliveries.sqlite3"));
/// ```
pub fn ledger_path(root: &Path) -> Result<PathBuf> {
    let dir = codetether_paths::data_dir_for_workspace(root)
        .context("cannot resolve the workspace data directory for the delivery ledger")?;
    Ok(dir.join(FILE))
}

pub(super) fn open(root: &Path) -> Result<Connection> {
    let file = ledger_path(root)?;
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create delivery ledger directory {}", parent.display()))?;
    }
    let db = Connection::open(&file)
        .with_context(|| format!("open delivery ledger {}", file.display()))?;
    db.busy_timeout(std::time::Duration::from_secs(5))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600))?;
    }
    db.pragma_update(None, "journal_mode", "WAL")?;
    db.pragma_update(None, "synchronous", "FULL")?;
    super::schema::upgrade(&db)?;
    Ok(db)
}
