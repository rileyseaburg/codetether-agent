//! Pending-image attachment without a mutable session.
/// Implementation detail of [crate::declare_frontend_session_free].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_session_free_image {
    ($presentation:path, $image:path) => {
        mod image {
            use super::{App, Path};
            use $presentation as Presentation;
            pub(super) fn attach_image_command(app: &mut App, cwd: &Path, cleaned: &str) {
                $image(app, cwd, cleaned, Presentation::Pending);
            }
        }
    };
}
