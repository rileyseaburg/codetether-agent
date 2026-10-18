//! Application finalize operations for child turns.
/// Supporting implementation for [crate::impl_child_turn].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_child_finalize {
    ($host:ty, $session:ty, $store:path, $bus:path, $clear:path) => {
        mod finalize {
            use $bus as bus_publish;
            use $crate::__private::{PublishedAgent, Result, async_trait};
            use $store as store;
            #[async_trait]
            impl $crate::FinalizationHost for $host {
                type Session = $session;
                fn clear_steering(agent_id: &str) {
                    $clear(agent_id);
                }
                fn update_session(agent_id: &str, session: $session) {
                    store::update_session(agent_id, session);
                }
                async fn save(session: &$session) -> Result<()> {
                    session.save().await
                }
                fn lookup(agent_id: &str) -> Option<PublishedAgent> {
                    store::get(agent_id).map(|entry| PublishedAgent {
                        id: entry.session.id,
                        name: entry.name,
                        owner_session_id: entry.owner_session_id,
                    })
                }
                fn announce_result(agent_id: &str, response: &str, error: Option<&str>) {
                    bus_publish::announce_result(agent_id, response, error);
                }
                fn announce_done(agent_id: &str, success: bool, summary: String) {
                    bus_publish::announce_done(agent_id, success, summary);
                }
            }
        }
    };
}
