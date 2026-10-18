//! Frontend composer and sidecar field selection for image recovery.
/// Declare image recovery over the frontend's pending images and text sidecars.
/// # Examples
/// See the frontend image-sidecar recovery module for selected state types.
#[macro_export]
macro_rules! declare_frontend_image_recovery {
    ($app:path, $composer:path $(,)?) => {
        use $app as App;
        use $composer as ComposerState;
        pub(crate) fn recover_pasted_images(app: &mut App) -> usize {
            let mut state = $crate::image_recovery::State {
                composer: ComposerState {
                    images: &mut app.state.attachments.images,
                    input: &mut app.state.composer.input,
                    cursor: &mut app.state.composer.cursor,
                    status: &mut app.state.status,
                },
                pastes: &mut app.state.attachments.text,
            };
            $crate::image_recovery::recover_pasted_images(&mut state)
        }
    };
}
