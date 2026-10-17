//! Workspace refresh during child reload.

use crate::ChildSession;
use codetether_agent_lifecycle::ResumeConfig;
use codetether_agent_registry::AgentEntry;

pub(super) fn apply<S: ChildSession>(entry: &mut AgentEntry<S>, config: &ResumeConfig) -> bool {
    if entry.session.settings().workspace_pinned {
        return false;
    }
    let Some(workspace) = &config.workspace else {
        return false;
    };
    if entry.session.settings().directory.as_ref() == Some(workspace) {
        return false;
    }
    *entry.session.settings().directory = Some(workspace.clone());
    super::prompt::refresh(entry);
    true
}
