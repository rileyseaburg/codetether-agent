//! Native effects for shared chat publication and missing-provider reporting.
use crate::{
    session::ImageAttachment,
    tui::{app::state::App, worker_bridge::TuiWorkerBridge},
};
#[path = "chat_publication_host.rs"]
mod publication_host;
/// Push user text and image messages into the visible chat log.
pub(crate) fn push_user_messages(app: &mut App, prompt: &str, images: &[ImageAttachment]) {
    codetether_chat_submit::publication::push_user_messages(
        &mut app.state.messages,
        prompt,
        images,
    );
}
/// Report missing provider configuration through shared publication sequencing.
pub(super) async fn no_provider_error(app: &mut App, bridge: &Option<TuiWorkerBridge>) {
    codetether_chat_submit::publication::no_provider(&mut publication_host::Native { app, bridge })
        .await;
}
