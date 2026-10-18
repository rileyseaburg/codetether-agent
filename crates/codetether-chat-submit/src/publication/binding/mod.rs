//! Frontend message publication and missing-provider entry points.
mod host;
/// Declare message publication and missing-provider effects on frontend state.
/// # Examples
/// See the frontend chat helper module for its concrete types and callback.
#[macro_export]
macro_rules! declare_frontend_publication {
    ($app:path, $image:path, $bridge:path, $message:path, $stopped:path $(,)?) => {
        use $app as App;
        use $bridge as TuiWorkerBridge;
        use $image as ImageAttachment;
        $crate::__frontend_publication_host!($message, $stopped);
        /// Push user text and image messages into the visible chat log.
        pub(crate) fn push_user_messages(app: &mut App, prompt: &str, images: &[ImageAttachment]) {
            $crate::publication::push_user_messages(&mut app.state.messages, prompt, images);
        }
        /// Report missing provider configuration through shared publication sequencing.
        pub(super) async fn no_provider_error(app: &mut App, bridge: &Option<TuiWorkerBridge>) {
            $crate::publication::no_provider(&mut publication_host::Native { app, bridge }).await;
        }
    };
}
