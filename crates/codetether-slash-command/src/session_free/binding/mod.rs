//! Frontend bindings for UI-only commands while a session is checked out.
mod helpers;
mod host;
mod image;
/// Declare attachment and view operations that require no mutable session.
///
/// File and image attachment callbacks and model selection retain the existing
/// frontend state; the shared command classifier determines handled commands.
/// # Examples
/// See the frontend session-free command module for concrete callback paths.
#[macro_export]
macro_rules! declare_frontend_session_free {
    ($app:path, $registry:path, $mode:path, $presentation:path;
     $file:path, $model:path, $image:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $registry as ProviderRegistry;
        $crate::__frontend_session_free_host!($mode);
        $crate::__frontend_session_free_helpers!($file, $model);
        $crate::__frontend_session_free_image!($presentation, $image);
        /// Dispatch a UI-only command without accessing the current session.
        pub(super) fn run(
            app: &mut App,
            cwd: &Path,
            registry: &Option<Arc<ProviderRegistry>>,
            prompt: &str,
        ) -> bool {
            $crate::session_free::run(&mut host::Native { app, cwd, registry }, prompt)
        }
    };
}
