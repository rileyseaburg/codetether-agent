//! Application startup operations for child turns.
/// Supporting implementation for [crate::impl_child_turn].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_child_startup {
    ($host:ty, $session:ty, $store:path, $prepare:path, $ensure:path, $open:path, $announce:path) => {
        mod startup {
            use $crate::__private::{Params, Result, async_trait};
            use $store as store;
            #[async_trait]
            impl $crate::StartupHost for $host {
                fn first_session(agent_id: &str) -> Option<($session, String)> {
                    store::get(agent_id).map(|entry| (entry.session, entry.instructions))
                }
                async fn message_session(agent_id: &str, params: &Params) -> Result<$session> {
                    let session =
                        store::get_for_parent(agent_id, params.parent_session_id.as_deref())
                            .map(|entry| entry.session);
                    $prepare(agent_id, params, session).await
                }
                fn ensure_bus(session: &mut $session) {
                    $ensure(session);
                }
                fn open_steering(agent_id: &str) {
                    $open(agent_id);
                }
                fn announce_working(agent_id: &str, summary: &'static str) {
                    $announce(agent_id, summary);
                }
            }
        }
    };
}
