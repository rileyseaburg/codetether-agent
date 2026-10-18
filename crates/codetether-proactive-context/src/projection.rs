//! Map global prepared projections into the loaded transcript window.
use codetether_context_policy::summary_index::SummaryIndex;

pub(super) async fn read<S: crate::Source, H: crate::Host>(
    session: &S,
    host: &H,
) -> Option<SummaryIndex> {
    let value = host.projection(session.id(), "rlm").await.ok()??;
    codetether_prepared_context::window_index(
        &value,
        session.message_offset(),
        session.messages().len(),
    )
}
