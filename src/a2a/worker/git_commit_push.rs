//! Adapt repository publication progress to the worker task timeline.
use super::task_timeline;
use anyhow::Result;
use codetether_worktree::repository::publication::{self, PublicationCheckpoint};
use std::path::Path;
pub(super) async fn run(
    repo_path: &Path,
    task_id: &str,
    provenance: Option<&crate::provenance::ExecutionProvenance>,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<Option<String>> {
    publication::run(repo_path, task_id, provenance, |event| {
        timeline.checkpoint(match event {
            PublicationCheckpoint::Staging => task_timeline::TaskCheckpoint::CommitStaging,
            PublicationCheckpoint::Created => task_timeline::TaskCheckpoint::CommitCreated,
            PublicationCheckpoint::Pushing => task_timeline::TaskCheckpoint::CommitPushing,
            PublicationCheckpoint::Pushed => task_timeline::TaskCheckpoint::CommitPushed,
        });
    })
    .await
}
