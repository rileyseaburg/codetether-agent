//! Native read-only conversation and presentation bindings for side questions.
mod bindings;
use crate::{provider::ProviderRegistry, session::Session, tui::app::state::App};
use std::sync::Arc;
/// Run an ephemeral side question without mutating the active session.
pub(super) async fn run_ask(
    app: &mut App,
    session: &Session,
    registry: Option<&Arc<ProviderRegistry>>,
    question: &str,
) -> bool {
    codetether_side_question::run_ask(
        &mut bindings::Output(app),
        session.history(),
        session.metadata.model.as_deref(),
        registry,
        question,
    )
    .await
}
