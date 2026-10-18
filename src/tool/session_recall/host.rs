//! Native indexed search and bus type for shared recall synthesis.
use codetether_session_recall::{RecallHit, SearchOptions};
use std::path::Path;
/// Recall host retaining the native indexed search and session event type.
pub type ApplicationHost = codetether_session_recall::application::ApplicationHost<
    crate::session::SessionBus,
    IndexedSearch,
>;
/// Existing indexed recall query, with its workspace filters unchanged.
pub struct IndexedSearch;
#[async_trait::async_trait]
impl codetether_session_recall::application::Search for IndexedSearch {
    async fn search(
        workspace: &Path,
        query: &str,
        options: SearchOptions<'_>,
    ) -> anyhow::Result<Vec<RecallHit>> {
        crate::session::index::recall::search::run(
            workspace,
            query,
            crate::session::index::recall::search::SearchOptions {
                session_id: options.session_id,
                excluded_session: options.excluded_session,
                limit: options.limit,
                minimum_score: options.minimum_score,
            },
        )
        .await
    }
}
