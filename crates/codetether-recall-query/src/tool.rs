//! Adapt the recall query engine to the recall tool's search contract.
//!
//! [IndexedSearch] preserves query options and obtains storage through [crate::Host].
use codetether_session_recall::{RecallHit, SearchOptions};
use std::path::Path;
/// Search adapter using the application's existing stateless storage host.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_recall_query::Host + Default>() {
/// fn accepts_search<S: codetether_session_recall::application::Search>() {}
/// accepts_search::<codetether_recall_query::tool::IndexedSearch<H>>();
/// # }
/// ```
pub struct IndexedSearch<H>(std::marker::PhantomData<fn() -> H>);
#[async_trait::async_trait]
impl<H: crate::Host + Default> codetether_session_recall::application::Search for IndexedSearch<H> {
    async fn search(
        workspace: &Path,
        query: &str,
        options: SearchOptions<'_>,
    ) -> anyhow::Result<Vec<RecallHit>> {
        crate::run(
            workspace,
            query,
            crate::SearchOptions {
                session_id: options.session_id,
                excluded_session: options.excluded_session,
                limit: options.limit,
                minimum_score: options.minimum_score,
            },
            &H::default(),
        )
        .await
    }
}
