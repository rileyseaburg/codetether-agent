//! Prepared provider, tool registry, and checkout identity for an ephemeral run.

use crate::provider::Provider;
use crate::tool::ToolRegistry;
use std::{path::PathBuf, sync::Arc};

pub(in crate::tool::agent::spawn) struct Setup {
    pub provider: Arc<dyn Provider>,
    pub model: String,
    pub prompt: String,
    pub registry: Arc<ToolRegistry>,
    pub workspace: PathBuf,
    pub handoff: Option<super::super::workspace::Handoff>,
}
