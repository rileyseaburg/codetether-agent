//! Network-scoped Git finalization dispatch for completed A2A tasks.

use super::task_timeline;
use crate::session::Session;
use anyhow::Result;
use std::path::Path;

pub(super) async fn run(
    directory: &Path,
    task_id: &str,
    session: &Session,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<Option<String>> {
    super::super::git_commit_push::run(
        directory,
        task_id,
        session.metadata.provenance.as_ref(),
        timeline,
        session.metadata.allow_network,
    )
    .await
}
