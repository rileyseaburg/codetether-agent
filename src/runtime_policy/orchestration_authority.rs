//! Workspace-bound delegation for approved orchestration subprocesses.

#[path = "orchestration_authority_drop.rs"]
mod permit_drop;
#[path = "orchestration_authority_scope.rs"]
mod scope;
pub(crate) use scope::{allows, bind};

use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

const FIELD: &str = "__ct_orchestration_authority";
static ACTIVE: OnceLock<Mutex<HashMap<PathBuf, (String, usize)>>> = OnceLock::new();

pub(crate) struct Permit {
    workspace: PathBuf,
    token: String,
}

pub(crate) fn issue(workspace: &Path) -> Result<Permit> {
    let workspace = workspace.canonicalize()?;
    let mut active = map()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let entry = active
        .entry(workspace.clone())
        .or_insert_with(|| (uuid::Uuid::new_v4().to_string(), 0));
    entry.1 += 1;
    Ok(Permit {
        workspace,
        token: entry.0.clone(),
    })
}

fn map() -> &'static Mutex<HashMap<PathBuf, (String, usize)>> {
    ACTIVE.get_or_init(|| Mutex::new(HashMap::new()))
}
