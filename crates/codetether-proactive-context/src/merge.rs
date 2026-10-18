//! Prefer projections, then accept only prefix-compatible sidecar indexes.
use codetether_context_policy::summary_index::SummaryIndex;

/// Read a projection first, falling back to a prefix-compatible sidecar.
/// # Arguments
/// Receives the loaded session window and native storage host.
/// # Returns
/// Returns the selected index, or an empty index when neither source is valid.
/// # Examples
/// ```no_run
/// # async fn example(s: &impl codetether_proactive_context::Source, h: &impl codetether_proactive_context::Host) {
/// let index = codetether_proactive_context::prepared_index(s, h).await;
/// # }
/// ```
pub async fn prepared_index<S: crate::Source, H: crate::Host>(
    session: &S,
    host: &H,
) -> SummaryIndex {
    if let Some(index) = super::projection::read(session, host).await {
        return index;
    }
    let Some(prepared) = host.read_prepared(session.id()).await else {
        return SummaryIndex::new();
    };
    let valid = codetether_prepared_context::matches(
        session.messages(),
        prepared.message_count,
        prepared.fingerprint,
    );
    if valid {
        prepared.index
    } else {
        SummaryIndex::new()
    }
}
