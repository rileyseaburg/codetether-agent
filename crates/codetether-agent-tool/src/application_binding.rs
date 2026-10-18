//! Application action routing and the public agent-tool specialization.

/// Declare the agent tool using existing application action services.
///
/// Modules are selected for persistence, spawning, messages, control handlers,
/// status and durable lifecycle operations. Hydration preserves errors and
/// discards the successful count to satisfy the action host's unit result.
/// # Examples
/// ```text
/// codetether_agent_tool::declare_application_tool!(
///     persistence, spawn, message, handlers, status, lifecycle,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_tool {
    ($persistence:path, $spawn:path, $message:path, $handlers:path, $status:path, $lifecycle:path $(,)?) => {
        mod host {
            use $crate::__private::{Params, Result, ResumeConfig, ToolResult, async_trait};
            use $handlers as handlers;
            use $lifecycle as thread_lifecycle;
            use $message as message;
            use $persistence as persistence;
            use $spawn as spawn;
            use $status as status;

            pub struct ApplicationHost;
            #[async_trait]
            impl $crate::Host for ApplicationHost {
                async fn hydrate(parent: Option<&str>) -> Result<()> {
                    persistence::hydrate_parent(parent).await.map(|_| ())
                }
                async fn spawn(params: &Params) -> Result<ToolResult> {
                    spawn::handle_spawn(params).await
                }
                async fn message(params: &Params) -> Result<ToolResult> {
                    message::handle_message(params).await
                }
                async fn list(owner: Option<&str>) -> Result<ToolResult> {
                    handlers::handle_list(owner).await
                }
                fn status(owner: Option<&str>) -> ToolResult {
                    status::handle_status(owner)
                }
                async fn interrupt(
                    target: &str,
                    owner: Option<&str>,
                    marker: bool,
                ) -> Result<ToolResult> {
                    handlers::handle_interrupt(target, owner, marker).await
                }
                async fn close(target: &str, owner: Option<&str>) -> Result<ToolResult> {
                    thread_lifecycle::close(target, owner).await
                }
                async fn resume(
                    target: &str,
                    owner: Option<&str>,
                    config: ResumeConfig,
                ) -> Result<ToolResult> {
                    thread_lifecycle::resume(target, owner, config).await
                }
                async fn kill(target: &str, owner: Option<&str>) -> Result<ToolResult> {
                    handlers::handle_kill(target, owner).await
                }
            }
        }

        pub(super) use host::ApplicationHost;

        /// Native agent-management tool with the application's action services.
        pub type AgentTool = $crate::AgentTool<ApplicationHost>;

        /// Unit-style constructor retained for existing tool registrations.
        #[allow(non_upper_case_globals)]
        pub const AgentTool: AgentTool = AgentTool::new();
    };
}
