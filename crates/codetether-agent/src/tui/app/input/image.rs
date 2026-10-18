//! Frontend type selection for pending-image attachment and feedback.
codetether_image::declare_frontend_image_attachment!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);

#[cfg(test)]
include!("image_test_imports.rs");
