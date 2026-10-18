//! File attachment and model-picker callbacks without session ownership.
/// Implementation detail of [crate::declare_frontend_session_free].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_session_free_helpers {
    ($file:path, $model:path) => {
        mod helpers {
            use super::{App, Arc, Path, ProviderRegistry};
            /// Attach a file to the input, or open the file picker when `cleaned` is empty.
            pub(super) fn attach_file_command(app: &mut App, cwd: &Path, cleaned: &str) {
                $file(app, cwd, cleaned);
            }
            /// Open the shared model picker without a session.
            pub(super) fn open_model_view(app: &mut App, registry: &Option<Arc<ProviderRegistry>>) {
                $model(app, registry.as_ref());
            }
        }
    };
}
