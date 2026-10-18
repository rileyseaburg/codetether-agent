//! Native host binding for shared recall query dispatch.
use super::hit::RecallHit;
use anyhow::Result;
pub(crate) use codetether_recall_query::SearchOptions;
use std::path::Path;
pub(crate) async fn run(
    workspace: &Path,
    query: &str,
    options: SearchOptions<'_>,
) -> Result<Vec<RecallHit>> {
    codetether_recall_query::run(workspace, query, options, &super::storage_host::NativeHost).await
}
