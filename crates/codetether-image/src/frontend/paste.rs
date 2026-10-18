//! Frontend composer field access for standalone and embedded image URLs.
/// Declare image-URL paste ingestion over the frontend composer state.
/// # Examples
/// See the frontend image-data paste module for invocation and tests.
#[macro_export]
macro_rules! declare_frontend_image_paste {
    ($app:path $(,)?) => {
        use $app as App;
        /// Handle a paste that is entirely one image data URL.
        ///
        /// # Returns
        ///
        /// `true` when the paste was consumed — either attached as an image, or
        /// recognised as an image that cannot be attached (for example a screenshot
        /// over the size cap) and reported to the user. Consuming a rejected image
        /// keeps megabytes of base64 out of the large-paste text sidecar, where the
        /// failure would otherwise be silent.
        pub(crate) fn try_attach_data_url(app: &mut App, text: &str) -> bool {
            $crate::paste::try_attach_data_url(&mut state(app), text)
        }

        /// Pull every embedded image data URL out of `app.state.composer.input`, attach each,
        /// and replace the input with the remaining text. Returns the count attached.
        pub(crate) fn drain_embedded_images(app: &mut App) -> usize {
            $crate::paste::drain_embedded_images(&mut state(app))
        }

        fn state(app: &mut App) -> $crate::paste::State<'_> {
            $crate::paste::State {
                images: &mut app.state.attachments.images,
                input: &mut app.state.composer.input,
                cursor: &mut app.state.composer.cursor,
                status: &mut app.state.status,
            }
        }
    };
}
