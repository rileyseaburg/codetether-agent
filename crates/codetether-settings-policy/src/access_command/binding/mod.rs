//! Frontend type and callback selection for access-mode command policy.
mod host;
/// Declare access-mode command dispatch over existing frontend session state.
///
/// Verifier-command precedence, config application and approval release remain
/// in the shared command policy and selected frontend services.
/// # Examples
/// See the frontend access-mode module for its concrete resource bindings.
#[macro_export]
macro_rules! declare_frontend_access_command {
    ($app:path, $session:path, $registry:path, $config:path, $message:path, $kind:path;
     $verifier:path, $release:path $(,)?) => {
        use $app as App;
        use $session as Session;
        use $registry as ProviderRegistry;
        use ::std::{path::Path, sync::Arc};
        $crate::__frontend_access_host!($config, $message, $kind; $verifier, $release);
        /// Handle an access-mode or verifier-model command.
        pub(super) async fn run(
            app: &mut App,
            cwd: &Path,
            session: &mut Session,
            registry: Option<&Arc<ProviderRegistry>>,
            prompt: &str,
        ) -> bool {
            $crate::access_command::run(
                &mut host::Native {
                    app,
                    session,
                    registry,
                },
                cwd,
                prompt,
            )
            .await
        }
    };
}
