//! Application execution, messaging and wait-service host declarations.

/// Bind collaboration tools to existing application services.
///
/// Selects the agent tool and callbacks for canonical lookup, activation,
/// context conversion, steering, queueing, listing, snapshots, status channels
/// and parent activity. Shared collaboration logic retains sequencing.
/// # Examples
/// ```text
/// codetether_collaboration_tools::declare_application_host!(
///     AgentTool; canonical, open, resume_config, steer, queue, list, find,
///     subscribe, set_status, activity,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_host {
    ($agent:ty; $canonical:path, $open:path, $resume:path, $steer:path, $queue:path, $list:path, $find:path, $subscribe:path, $set:path, $activity:path $(,)?) => {
        use $crate::__private::{
            Instant, Result, Tool, ToolResult, Value, async_trait, json, watch,
        };
        use $crate::{Activity, EnsureOpen, Route, RuntimeContext, ThreadStatus, WaitAgent};

        pub struct ApplicationHost;
        #[async_trait]
        impl $crate::Host for ApplicationHost {
            async fn execute(payload: Value) -> Result<ToolResult> {
                <$agent>::new().execute(payload).await
            }
            fn canonical(owner: &str, agent_id: &str) -> Result<Option<String>> {
                $canonical(owner, agent_id)
            }
        }

        #[async_trait]
        impl $crate::MessagingHost for ApplicationHost {
            async fn open(
                target: &str,
                owner: Option<&str>,
                context: &RuntimeContext,
            ) -> Result<EnsureOpen> {
                $open(target, owner, $resume(context)).await
            }
            async fn steer(target: &str, owner: Option<&str>, message: &str) -> Route {
                $steer(target, owner, message).await
            }
            async fn queue_only(
                target: &str,
                owner: Option<&str>,
                message: String,
            ) -> Result<ToolResult> {
                $queue(target, owner, message).await
            }
            fn list(current: &str, path_prefix: Option<&str>) -> Result<Value> {
                Ok(json!($list(current, path_prefix)?))
            }
        }

        #[async_trait]
        impl $crate::WaitHost for ApplicationHost {
            fn find_agent(target: &str, owner: &str) -> Option<WaitAgent> {
                $find(target, owner).map(|agent| WaitAgent {
                    id: agent.id,
                    is_processing: agent.is_processing,
                })
            }
            fn subscribe(agent_id: &str) -> Option<watch::Receiver<ThreadStatus>> {
                $subscribe(agent_id)
            }
            fn set_status(agent_id: &str, status: ThreadStatus) {
                $set(agent_id, status)
            }
            async fn activity(owner: &str, deadline: Instant) -> Option<Activity> {
                $activity(owner, deadline).await
            }
        }
    };
}
