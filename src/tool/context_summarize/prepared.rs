//! Owned snapshot of the inputs used by the existing summary producer.
pub use codetether_context_summarize::production::Prepared;
pub(super) fn capture(session: &crate::session::Session) -> Prepared {
    let messages = session.messages.clone();
    let generation = session.summary_index.generation();
    let rlm = session.metadata.rlm.clone();
    let session_id = session.id.clone();
    let sub_provider = session.metadata.subcall_provider.clone();
    let sub_model = session.metadata.subcall_model_name.clone();

    Prepared::new(
        messages,
        generation,
        rlm,
        session_id,
        sub_provider,
        sub_model,
    )
}
