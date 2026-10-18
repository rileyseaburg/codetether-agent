//! Application registry, steering and session bindings for message delivery.

/// Declare active steering and durable idle delivery for existing services.
///
/// Arguments select the function visibility, session type, live registry and
/// steering module. Human-message bookkeeping and persistence go through the
/// normal session methods; live snapshots are updated by the shared workflow.
///
/// # Examples
/// ```text
/// codetether_agent_communication::declare_application_communication!(
///     pub(crate), Session, store, steering,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_communication {
    ($visibility:vis, $session:ty, $store:path, $steering:path $(,)?) => {
        $visibility use $crate::Route;

        mod host {
            use $store as store;
            use $steering as steering;
            use steering::SteeringInput;

            mod session {
                pub(super) fn add_human_message(session: &mut $session, message: $crate::__private::Message) {
                    session.add_human_message(message);
                }
                pub(super) async fn save(session: &$session) -> $crate::__private::Result<()> {
                    session.save().await
                }
            }

            pub struct ApplicationHost;
            #[$crate::__private::async_trait]
            impl $crate::Host for ApplicationHost {
                type Session = $session;
                fn resolve(target: &str, owner: Option<&str>) -> Option<String> {
                    store::scope::resolve_for_parent(target, owner).map(|(id, _)| id)
                }
                fn session(id: &str) -> Option<$session> {
                    store::get(id).map(|entry| entry.session)
                }
                fn push(id: &str, message: &str) -> bool {
                    let input = SteeringInput::new(message.into(), Vec::new());
                    steering::push(id, input)
                }
                fn add_human_message(session: &mut $session, message: $crate::__private::Message) {
                    session::add_human_message(session, message);
                }
                async fn save(session: &$session) -> $crate::__private::Result<()> {
                    session::save(session).await
                }
                fn update_session(id: &str, session: $session) {
                    store::update_session(id, session);
                }
            }
        }

        $visibility async fn steer(target: &str, owner: Option<&str>, message: &str) -> Route {
            $crate::steer::<host::ApplicationHost>(target, owner, message).await
        }

        $visibility async fn queue_only(
            target: &str,
            owner: Option<&str>,
            message: String,
        ) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::queue_only::<host::ApplicationHost>(target, owner, message).await
        }
    };
}
