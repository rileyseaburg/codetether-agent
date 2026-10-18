//! Validate and dispatch the managed child spawn command.
use codetether_tool_core::ToolResult;
use std::future::Future;
/// Session-bound operations used by the spawn command.
/// # Implementors
/// Supply native session/workspace context and the existing child watch transition.
pub trait Host {
    /// Display command validation feedback.
    fn status(&mut self, text: String);
    /// Execute the canonical child spawn request.
    fn spawn(&mut self, name: &str, instructions: &str) -> impl Future<Output = ToolResult>;
    /// Focus the child and show its live transcript.
    fn watch(&mut self, name: &str);
    /// Present the spawn result.
    fn present(&mut self, result: &ToolResult, success: String, failure: &str);
}
/// Spawn and immediately dispatch a managed child agent.
/// # Arguments
/// `app` supplies session-bound operations; `rest` contains spawn operands.
/// # Returns
/// Updates validation status or presents the result after dispatch.
/// # Examples
/// ```no_run
/// async fn spawn<H: codetether_agent_command::spawn::Host>(host: &mut H) {
///     codetether_agent_command::spawn::handle(host, "worker inspect").await;
/// }
/// ```
pub async fn handle<H: Host>(app: &mut H, rest: &str) {
    let Some(args) = codetether_agent_spawn::parse_spawn_args(rest) else {
        app.status("Usage: /spawn <name> [instructions]".to_string());
        return;
    };
    if args.parent.is_some() {
        app.status("Nested --parent spawning is not supported by the managed runtime".into());
        return;
    }
    let instructions = codetether_agent_identity::system_prompt(&args.name, &args.instructions);
    let result = app.spawn(&args.name, &instructions).await;
    if result.success {
        app.watch(&args.name);
    }
    app.present(
        &result,
        format!("Managed sub-agent @{} started", args.name),
        "Sub-agent spawn failed",
    );
}
