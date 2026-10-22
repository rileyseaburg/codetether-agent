//! Map global prepared projections into the loaded transcript window.
use codetether_context_policy::summary_index::SummaryIndex;

pub(super) async fn read<S: crate::Source, H: crate::Host>(
    session: &S,
    host: &H,
) -> Option<SummaryIndex> {
    let name = codetether_context_policy::summary_index::projection_name();
    let value = host.projection(session.id(), &name).await.ok()??;
    codetether_prepared_context::window_index(
        &value,
        session.message_offset(),
        session.messages().len(),
    )
}
