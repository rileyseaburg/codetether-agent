//! Arrange a held goal with a deterministic mock provider.

use crate::provider::ProviderRegistry;
use crate::session::tasks::runtime::answer_review;
use crate::tui::app::{session_runtime::SessionSlot, state::App};
use std::sync::Arc;

pub(super) async fn setup(fail: bool) -> (App, SessionSlot, Option<Arc<ProviderRegistry>>) {
    let mut session = crate::session::tasks::answer_review_test_support::session().await;
    session.metadata.model = Some("answer-test/model".into());
    answer_review::begin(&session.id, "Why?").await.unwrap();
    let mut registry = ProviderRegistry::new();
    registry.register(Arc::new(super::provider::ReplyProvider(fail)));
    let mut app = App::default();
    app.state.session_id = Some(session.id.clone());
    (app, SessionSlot::new(session), Some(Arc::new(registry)))
}
