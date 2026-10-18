//! Application session and store operations for durable child registration.

/// Declare durable registration using the existing session and store owners.
///
/// The shared registration workflow retains write ordering and reservation
/// accounting. The supplied stores perform lineage lookup, manifest writes and
/// insertion; sessions save through their normal persistence method.
///
/// # Examples
/// ```text
/// codetether_agent_spawn::declare_application_registration!(
///     pub(super), Session, store, persistence,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_registration {
    ($visibility:vis, $session:ty, $store:path, $persistence:path $(,)?) => {
        mod host {
            use $store as store;
            use $persistence as persistence;

            pub struct ApplicationStore;
            #[$crate::__private::async_trait]
            impl $crate::Registration for ApplicationStore {
                type Session = $session;
                async fn save_session(session: &$session) -> $crate::__private::Result<()> {
                    session.save().await
                }
                fn lineage(parent_id: Option<&str>) -> (Option<String>, u8) {
                    store::lineage_for_session(parent_id)
                }
                async fn save_entry(name: &str, entry: &store::AgentEntry) -> $crate::__private::Result<()> {
                    persistence::save(name, entry).await
                }
                fn insert_reserved(entry: store::AgentEntry) {
                    store::insert_reserved(entry);
                }
            }
        }

        $visibility async fn persist_spawned_agent(
            request: &$crate::__private::SpawnRequest<'_>,
            session: $session,
        ) -> $crate::__private::Result<String> {
            $crate::persist_spawned_agent::<host::ApplicationStore>(request, session).await
        }
    };
}
