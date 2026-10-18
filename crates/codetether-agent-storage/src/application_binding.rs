//! Application session, registry and mailbox selection for durable agent storage.

/// Bind durable storage operations to existing application services.
///
/// The visibility argument controls child lifecycle and lookup access. The
/// remaining arguments select the session type, live-registry module and
/// durable-mailbox module. Registry mutations retain residency bookkeeping;
/// mailbox operations retain their normal persistence and dispatch paths.
///
/// # Examples
/// ```text
/// codetether_agent_storage::declare_application_storage!(
///     pub(crate); Session, live_registry, mailbox,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_storage {
    ($visibility:vis; $session:ty, $store:path, $mailbox:path $(,)?) => {
        mod host {
            use $store as store;
            use $mailbox as message_queue;

            pub struct ApplicationHost;

            #[$crate::__private::async_trait]
            impl $crate::RuntimeHost for ApplicationHost {
                type Session = $session;
                async fn resume(id: &str) -> $crate::__private::Result<$session> {
                    <$session>::resume(id).await
                }
                fn get(id: &str) -> Option<store::AgentEntry> {
                    store::get(id)
                }
                fn insert(entry: store::AgentEntry) {
                    store::insert(entry);
                }
                fn insert_reserved(entry: store::AgentEntry) {
                    store::insert_reserved(entry);
                }
                fn remove(id: &str) {
                    store::remove(id);
                }
                fn reparent_owner(from: &str, to: &str) {
                    store::reparent_owner(from, to);
                }
            }

            #[$crate::__private::async_trait]
            impl $crate::Host for ApplicationHost {
                async fn hydrate_mailbox(id: &str) -> $crate::__private::Result<bool> {
                    message_queue::hydrate(id).await
                }
                fn dispatch_next(id: String) {
                    message_queue::dispatch_next(id);
                }
                async fn reparent_mailbox(id: &str, from: &str, to: &str) -> $crate::__private::Result<()> {
                    message_queue::reparent_owner(id, from, to).await
                }
            }
        }

        $crate::__declare_storage_operations!($visibility; $session, host::ApplicationHost);
    };
}
