//! Application ownership checks and queued-turn execution bindings.

/// Declare native mailbox operations with an ownership store and turn executor.
///
/// The executor receives the original message, images, guard and receipt using
/// the reconstructed detached-message parameters. Storage remains owned by the
/// existing native mailbox and shared queue services.
/// # Examples
/// ```text
/// codetether_native_agent::declare_application_mailbox!(
///     pub(crate), store, execute_message,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_mailbox {
    ($visibility:vis, $store:path, $execute:path $(,)?) => {
        use $store as store;
        pub(crate) use $crate::__private::child_mailbox::{Submission, pending};

        mod host {
            pub(super) type ApplicationHost = $crate::Mailbox<Execution>;
            pub(super) struct Execution;
            #[$crate::__private::async_trait]
            impl $crate::MailboxExecution for Execution {
                async fn execute(agent_id: String, item: $crate::__private::child_mailbox::Item, guard: $crate::__private::AgentRunGuard) -> $crate::__private::Result<$crate::__private::ToolResult> {
                    let input = $crate::queued_mailbox_params(&item);
                    $execute(agent_id, item.message, item.images, &input, guard, Some(item.id.clone())).await
                }
            }
        }
        use host::ApplicationHost as H;

        $visibility async fn enqueue(agent_id: &str, message: String, input: &$crate::__private::Params) -> $crate::__private::Result<Submission> {
            store::get_for_parent(agent_id, input.parent_session_id.as_deref())
                .ok_or_else(|| $crate::__private::anyhow!("Agent {agent_id} not found"))?;
            $crate::__private::child_mailbox::enqueue::<H>(
                agent_id,
                message,
                $crate::__private::child_mailbox::EnqueueInput {
                    images: &input.message_images,
                    parent_id: input.parent_session_id.as_deref(),
                    prior_context: input.parent_prior_context_allowed,
                },
            ).await
        }

        $visibility async fn finished(agent_id: String, receipt_id: Option<String>, guard: $crate::__private::AgentRunGuard) {
            $crate::__private::child_mailbox::finished::<H>(agent_id, receipt_id, guard).await
        }
        pub(crate) fn dispatch_next(agent_id: String) {
            $crate::__private::child_mailbox::dispatch_next::<H>(agent_id)
        }
        pub(crate) async fn hydrate(agent_id: &str) -> $crate::__private::Result<bool> {
            $crate::__private::child_mailbox::hydrate::<H>(agent_id).await
        }
        pub(crate) async fn clear(agent_id: &str) -> $crate::__private::Result<()> {
            $crate::__private::child_mailbox::clear::<H>(agent_id).await
        }
        pub(crate) async fn reparent_owner(agent_id: &str, from: &str, to: &str) -> $crate::__private::Result<()> {
            $crate::__private::child_mailbox::reparent_owner::<H>(agent_id, from, to).await
        }

        #[cfg(test)]
        pub(crate) async fn claim_for_test(agent_id: &str) -> $crate::__private::Result<Option<(String, String)>> {
            Ok($crate::__private::child_mailbox::claim::<H>(agent_id).await?.map(|item| (item.id, item.message)))
        }
        #[cfg(test)]
        pub(crate) async fn reset_for_test() {
            $crate::__private::child_mailbox::reset_cache().await
        }
        #[cfg(test)]
        pub(crate) async fn ack_for_test(agent_id: &str, id: &str) -> $crate::__private::Result<()> {
            $crate::__private::child_mailbox::ack::<H>(agent_id, id).await
        }
    };
}
