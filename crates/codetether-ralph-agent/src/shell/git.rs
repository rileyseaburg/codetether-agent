//! Bind Ralph Git checks to the same application command policy as its shell tools.
use codetether_application_registry::Application;
use codetether_tool_core::ToolResult;
use std::marker::PhantomData;
/// Git policy using the application's authoritative Bash invocation check.
/// # Examples
/// ```no_run
/// # fn example<A: codetether_application_registry::Application>() {
/// fn accepts_policy<P: codetether_ralph_git::Host>() {}
/// accepts_policy::<codetether_ralph_agent::shell::GitPolicy<A>>();
/// # }
/// ```
pub struct GitPolicy<A>(PhantomData<fn() -> A>);
#[async_trait::async_trait]
impl<A: Application> codetether_ralph_git::Host for GitPolicy<A> {
    async fn check(args: serde_json::Value) -> anyhow::Result<Option<ToolResult>> {
        Ok(A::evaluate("bash", &args).await)
    }
}
