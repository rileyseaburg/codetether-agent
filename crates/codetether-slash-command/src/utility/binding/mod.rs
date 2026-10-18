//! Frontend session context and handlers for utility commands.
mod host;
mod review;
/// Declare utility-command routing and review-composer bindings.
///
/// Concrete frontend types remain selected by the caller. The shared utility
/// policy retains access-command precedence and alias dispatch ordering.
/// # Examples
/// See the frontend utility-command module for concrete handler selection.
#[macro_export]
macro_rules! declare_frontend_utility {
    ($app:path, $session:path, $registry:path, $message:path, $kind:path;
     $access:path, $status:path, $diff:path, $copy:path, $dispatch:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $registry as ProviderRegistry;
        use $session as Session;
        $crate::__frontend_utility_host!($access, $status, $diff, $copy, $dispatch);
        $crate::__frontend_utility_review!($message, $kind);
        /// Run utility-command parsing against the existing frontend session.
        pub(super) async fn run(
            app: &mut App,
            cwd: &Path,
            session: &mut Session,
            registry: Option<&Arc<ProviderRegistry>>,
            prompt: &str,
        ) -> bool {
            $crate::utility::run(
                &mut host::Native {
                    app,
                    cwd,
                    session,
                    registry,
                },
                prompt,
            )
            .await
        }
    };
}
