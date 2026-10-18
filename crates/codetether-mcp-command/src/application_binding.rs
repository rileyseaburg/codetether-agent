//! Frontend connection-registry and notice bindings for MCP commands.

/// Declare MCP command handling over the frontend's existing registry.
///
/// The notice callback retains the application's normal system-message path.
/// Command execution and connection behavior remain with the shared MCP code.
/// # Examples
/// ```text
/// codetether_mcp_command::declare_frontend_command!(App, push_system_message);
/// ```
#[macro_export]
macro_rules! declare_frontend_command {
    ($app:ty, $notice:path $(,)?) => {
        mod mcp_host {
            pub(super) struct Native<'a>(pub &'a mut $app);
            impl $crate::interactive::Host for Native<'_> {
                async fn execute(&mut self, raw: &str) -> $crate::CommandOutput {
                    $crate::execute(&self.0.state.mcp_registry, raw).await
                }
                fn set_status(&mut self, status: String) {
                    self.0.state.status = status;
                }
                fn push_system_message(&mut self, message: String) {
                    $notice(self.0, message);
                }
            }
        }
        async fn handle_mcp_command(app: &mut $app, raw: &str) {
            $crate::interactive::handle(&mut mcp_host::Native(app), raw).await;
        }
    };
}
