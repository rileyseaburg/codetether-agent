//! Application mailbox operations for child turns.
/// Supporting implementation for [crate::impl_child_turn].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_child_mailbox {
    ($host:ty, $queue:path) => {
        mod mailbox {
            use $crate::__private::{Params, Result, Submission, async_trait};
            use $queue as message_queue;
            #[async_trait]
            impl $crate::MailboxHost for $host {
                async fn enqueue(
                    agent_id: &str,
                    message: String,
                    params: &Params,
                ) -> Result<Submission> {
                    message_queue::enqueue(agent_id, message, params).await
                }
                fn dispatch_next(agent_id: String) {
                    message_queue::dispatch_next(agent_id);
                }
            }
        }
    };
}
