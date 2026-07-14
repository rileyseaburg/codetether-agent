//! Borrowable recall sources for workspace and direct-session queries.

use std::path::Path;
use std::sync::Arc;

use super::indexed_session::IndexedSession;

pub(super) enum SearchSource {
    Session(Vec<IndexedSession>),
    Workspace(Arc<Vec<IndexedSession>>),
}

impl SearchSource {
    pub(super) async fn load(workspace: &Path, session_id: Option<&str>) -> Self {
        match session_id {
            Some(id) => Self::Session(super::load::session(id).await.into_iter().collect()),
            None => Self::Workspace(super::load::workspace(workspace).await),
        }
    }

    pub(super) fn sessions(&self) -> &[IndexedSession] {
        match self {
            Self::Session(sessions) => sessions,
            Self::Workspace(sessions) => sessions,
        }
    }
}
