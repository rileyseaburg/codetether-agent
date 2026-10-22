//! Append runtime review before projection, as the native loop does.
use super::Session;
use codetether_agent::provider::Role;
pub(super) fn source(original: &Session) -> (Session, bool) {
    let mut source = Session {
        offset: original.offset,
        messages: original.messages.clone(),
        pages: original.pages[..original.messages.len()].to_vec(),
        summary_index: original.summary_index.clone(),
        background: original.background.clone(),
    };
    let mut ledger = codetether_repeat_guard::ProgressLedger::default();
    for (index, message) in source.messages.iter().enumerate() {
        if message.role == Role::User {
            ledger.human_input(message);
        }
        ledger.observe_message(message, source.offset + index);
    }
    let review = ledger.take_review(15);
    let due = review.is_some();
    if let Some(review) = review {
        let message = codetether_prompt_loop::response::corrective_message(&review);
        source
            .pages
            .push(codetether_context_policy::pages::classify(&message));
        source.messages.push(message);
    }
    (source, due)
}
