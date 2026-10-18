//! Frontend pending-image queue and visible attachment feedback.
/// Declare image attachment effects using the existing frontend message types.
/// # Examples
/// See the frontend image module for concrete type selection.
#[macro_export]
macro_rules! declare_frontend_image_attachment {
    ($app:path, $message:path, $kind:path $(,)?) => {
        use ::std::path::Path;
        use $app as App;
        use $crate::{
            ImageAttachment,
            interactive::{Presentation, View},
        };
        use $kind as MessageType;
        use $message as ChatMessage;
        struct ImageView<'a>(&'a mut App);
        impl View for ImageView<'_> {
            fn push_image(&mut self, image: ImageAttachment) -> usize {
                self.0.state.pending_images.push(image);
                self.0.state.pending_images.len()
            }
            fn set_status(&mut self, message: String) {
                self.0.state.status = message;
            }
            fn push_system_message(&mut self, message: String) {
                self.0
                    .state
                    .messages
                    .push(ChatMessage::new(MessageType::System, message));
                self.0.state.chat_viewport.scroll_to_bottom();
            }
        }
        pub(crate) fn attach_image_command(
            app: &mut App,
            cwd: &Path,
            cleaned: &str,
            mode: Presentation,
        ) {
            $crate::interactive::attach(&mut ImageView(app), cwd, cleaned, mode);
        }
    };
}
