//! Application runtime operations for child turns.
/// Supporting implementation for [crate::impl_child_turn].
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_child_runtime {
    ($host:ty, $session:ty, $event:ty, $begin:path, $finish:path) => {
        mod runtime {
            use $crate::__private::{AgentRunGuard, Result, async_trait};
            use $crate::Started;
            #[async_trait]
            impl $crate::TurnHost for $host {
                async fn begin(agent_id: &str) -> Result<Started<$event, $session>> {
                    $begin(agent_id).await
                }
                async fn finished(agent_id: String, receipt: Option<String>, guard: AgentRunGuard) {
                    $finish(agent_id, receipt, guard).await;
                }
            }
        }
    };
}
