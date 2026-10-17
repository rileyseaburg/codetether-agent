//! Bind computer grants to the existing platform backend and runtime policy.
#[derive(Clone, Debug)]
pub struct ApplicationHost;
#[async_trait::async_trait]
impl codetether_tetherscript_computer::Host for ApplicationHost {
    type Input = crate::tool::computer_use::input::ComputerUseInput;
    async fn dispatch(input: &Self::Input) -> anyhow::Result<crate::tool::ToolResult> {
        crate::tool::computer_use::platform::dispatch(input).await
    }
    async fn evaluate_tool_invocation(
        tool: &str,
        args: &serde_json::Value,
    ) -> Option<crate::tool::ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(tool, args).await
    }
}
