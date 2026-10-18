//! Bind computer grants to the existing platform backend and runtime policy.
#[derive(Clone, Debug)]
/// Application computer-use backend and authorization for plugin capabilities.
///
/// # Examples
/// ```rust
/// let _host=codetether_native_tetherscript::ApplicationHost;
/// ```
pub struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_tetherscript_computer::Host for ApplicationHost {
    type Input = codetether_computer_tool::input::ComputerUseInput;
    async fn dispatch(input: &Self::Input) -> anyhow::Result<codetether_tool_core::ToolResult> {
        codetether_computer_tool::platform::dispatch(input).await
    }
    async fn evaluate_tool_invocation(
        tool: &str,
        args: &serde_json::Value,
    ) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(tool, args).await
    }
}
