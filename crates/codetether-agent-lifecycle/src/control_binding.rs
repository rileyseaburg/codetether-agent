//! Application session and storage bindings for explicit child control.

/// Bind interruption and removal to existing application services.
///
/// Arguments select the host name, session, persistence, live registry, mailbox,
/// terminal announcement and trace cleanup. Session markers use the session's
/// normal append/save methods and update the live snapshot only after saving.
///
/// # Examples
/// ```text
/// codetether_agent_lifecycle::declare_control_host!(
///     pub struct Native; Session, persistence, store, mailbox, announce, clear,
/// );
/// ```
#[macro_export]
macro_rules! declare_control_host {
    ($visibility:vis struct $host:ident; $(,)? $session:ty, $persistence:path, $store:path, $mailbox:path, $announce:path, $clear:path $(,)?) => {
        use $persistence as persistence;
        use $store as store;
        use $mailbox as message_queue;

        /// Application services used by child interruption and removal.
        $visibility struct $host;

        #[$crate::__private::async_trait]
        impl $crate::ControlHost for $host {
            type Session = $session;
            fn resolve(target: &str, parent: Option<&str>) -> Option<(String, store::AgentEntry)> {
                store::scope::resolve_for_parent(target, parent)
            }
            fn get(agent_id: &str) -> Option<store::AgentEntry> {
                store::get(agent_id)
            }
            async fn clear_mailbox(agent_id: &str) -> $crate::__private::Result<()> {
                message_queue::clear(agent_id).await
            }
            async fn remove(entry: &store::AgentEntry) -> $crate::__private::Result<()> {
                persistence::remove(entry).await
            }
            fn announce_done(agent_id: &str, success: bool, summary: &str) {
                $announce(agent_id, success, summary);
            }
            fn remove_runtime(agent_id: &str) {
                store::remove(agent_id);
                $clear(agent_id);
            }
        }

        #[$crate::__private::async_trait]
        impl $crate::MarkerHost for $host {
            fn add_message(session: &mut $session, message: $crate::__private::Message) {
                session.add_message(message);
            }
            async fn save(session: &$session) -> $crate::__private::Result<()> {
                session.save().await
            }
            fn update_session(agent_id: &str, session: $session) {
                store::update_session(agent_id, session);
            }
        }
    };
}
