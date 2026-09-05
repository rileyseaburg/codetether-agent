//! Workspace refresh during child reload.

use super::super::ResumeConfig;
use crate::tool::agent::store::AgentEntry;

pub(super) fn apply(entry: &mut AgentEntry, config: &ResumeConfig) -> bool {
    if entry.session.metadata.workspace_pinned {
        return false;
    }
    let Some(workspace) = &config.workspace else {
        return false;
    };
    if entry.session.metadata.directory.as_ref() == Some(workspace) {
        return false;
    }
    entry.session.metadata.directory = Some(workspace.clone());
    super::prompt::refresh(entry);
    true
}
