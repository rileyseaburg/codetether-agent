//! Host-side collection of LSP diagnostics for the Sol reviewer.

use anyhow::Result;
use serde_json::Value;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::session::Session;
use crate::session::helper::validation::{
    ValidationReport, build_validation_report, track_touched_files,
};

/// Validate only files touched through successful structured worker tools.
pub(super) async fn collect(
    workspace: &Path,
    session: &Session,
) -> Result<Option<ValidationReport>> {
    let mut touched = HashSet::<PathBuf>::new();
    for tool_use in session.tool_uses.iter().filter(|tool_use| tool_use.success) {
        let input = serde_json::from_str(&tool_use.input).unwrap_or(Value::Null);
        track_touched_files(&mut touched, workspace, &tool_use.name, &input, None);
    }
    build_validation_report(workspace, &touched, &HashSet::new()).await
}
