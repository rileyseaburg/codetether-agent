//! Application construction and execution bindings for shared spawn sequencing.

/// Declare a spawn entry point using the application's existing services.
///
/// Arguments select visibility, session type, capacity reservation, session
/// creation, persistence, validation, identity reservation, kickoff and ephemeral
/// execution. The shared orchestrator retains all sequencing and guard ownership.
///
/// # Examples
/// ```text
/// codetether_agent_spawn::declare_application_spawn!(
///     pub(super); Session, reserve, create, persist, validate, identity,
///     kick_off, ephemeral,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_spawn {
    ($visibility:vis; $session:ty, $reserve:path, $create:path, $persist:path, $validate:path, $identity:path, $kickoff:path, $ephemeral:path $(,)?) => {
        mod host {
            pub struct ApplicationHost;

            #[$crate::__private::async_trait]
            impl $crate::Construction for ApplicationHost {
                type Session = $session;
                async fn reserve_capacity() -> ::std::result::Result<$crate::__private::Slot, $crate::__private::ToolResult> {
                    $reserve(None).await
                }
                async fn create_session(
                    params: &$crate::__private::Params,
                    request: &$crate::__private::SpawnRequest<'_>,
                ) -> $crate::__private::Result<(Self::Session, $crate::__private::Handoff)> {
                    $create(params, request).await
                }
                async fn persist(request: &$crate::__private::SpawnRequest<'_>, session: Self::Session) -> $crate::__private::Result<String> {
                    $persist(request, session).await
                }
            }

            #[$crate::__private::async_trait]
            impl $crate::Host for ApplicationHost {
                async fn validate(request: &$crate::__private::SpawnRequest<'_>) -> ::std::result::Result<Option<String>, $crate::__private::ToolResult> {
                    $validate(request).await
                }
                async fn reserve_identity(name: &str, owner: Option<&str>) -> ::std::result::Result<$crate::__private::Reservation, $crate::__private::ToolResult> {
                    $identity(name, owner).await
                }
                async fn kick_off(agent_id: &str, detach: bool) -> $crate::__private::Result<$crate::__private::ToolResult> {
                    $kickoff(agent_id, detach).await
                }
                async fn ephemeral(request: &$crate::__private::SpawnRequest<'_>, warning: Option<&str>) -> $crate::__private::Result<$crate::__private::ToolResult> {
                    $ephemeral(request, warning).await
                }
            }
        }

        /// Spawn a child using the application's construction and execution services.
        $visibility async fn handle_spawn(params: &$crate::__private::Params) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::handle_spawn::<host::ApplicationHost>(params).await
        }
    };
}
