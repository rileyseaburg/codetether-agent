//! Helpers for chat prompt submission.
//!
//! Pushes user and image messages into the chat log and
//! handles the no-provider error case.
//!
//! # Examples
//!
//! ```ignore
//! push_user_messages(&mut app, "hello", &images);
//! no_provider_error(&mut app, &bridge).await;
//! ```

use crate::session::ImageAttachment;
use crate::tui::app::state::App;
use crate::tui::app::worker_bridge::handle_processing_stopped;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Push user text and image messages into the chat log.
///
/// # Examples
///
/// ```ignore
/// push_user_messages(&mut app, "hello", &[]);
/// ```
pub(super) fn push_user_messages(app: &mut App, prompt: &str, images: &[ImageAttachment]) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::User, prompt.to_string()));
    for image in images {
        app.state.messages.push(ChatMessage::new(
            MessageType::Image {
                url: image.data_url.clone(),
            },
            image.data_url.clone(),
        ));
    }
}

/// Show an error when no provider is configured.
///
/// # Examples
///
/// ```ignore
/// no_provider_error(&mut app, &bridge).await;
/// ```
pub(super) async fn no_provider_error(app: &mut App, bridge: &Option<TuiWorkerBridge>) {
    handle_processing_stopped(app, bridge).await;
    app.state.clear_request_timing();
    app.state.messages.push(ChatMessage::new(
        MessageType::Error,
        "No providers available. Configure credentials first.",
    ));
    app.state.status = "No providers configured".to_string();
    app.state.scroll_to_bottom();
}
