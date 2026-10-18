//! Application-local managed and isolated child creation entry points.

/// Declare session contracts and typed managed-child factory functions.
///
/// The session and prior-context callback retain the application's existing
/// fields, tracked-message operations and access policy.
///
/// # Examples
/// ```text
/// codetether_agent_session::declare_session_factory!(
///     pub(super); Session, prior_context_allowed,
/// );
/// ```
#[macro_export]
macro_rules! declare_session_factory {
    ($visibility:vis; $session:ty, $prior:path $(,)?) => {
        $crate::impl_managed_session!($session, $prior);

        $visibility async fn create_agent_session(
            name: &str,
            instructions: &str,
            model: &str,
            parent_workspace: Option<::std::path::PathBuf>,
            prior_context_allowed: bool,
        ) -> $crate::__private::Result<$session> {
            $crate::create::<$session>(
                name,
                instructions,
                model,
                parent_workspace,
                prior_context_allowed,
            )
            .await
        }

        $visibility async fn parent_prior_context_allowed(
            inherited: Option<bool>,
            parent_id: Option<&str>,
        ) -> bool {
            $crate::parent_prior_context_allowed::<$session>(inherited, parent_id).await
        }
    };
}

/// Declare a typed factory for a child in an isolated checkout.
///
/// # Examples
/// ```text
/// codetether_agent_session::declare_isolated_session!(pub(super); Session);
/// ```
#[macro_export]
macro_rules! declare_isolated_session {
    ($visibility:vis; $session:ty) => {
        $visibility async fn create(
            params: &$crate::__private::Params,
            request: &$crate::__private::SpawnRequest<'_>,
        ) -> $crate::__private::Result<($session, $crate::__private::Handoff)> {
            $crate::create_isolated::<$session>(params, request).await
        }
    };
}
