pub async fn runtime_denial(tool_name: &str, args: &serde_json::Value) -> Option<String> {
    crate::runtime_policy::evaluate_tool_invocation(tool_name, args)
        .await
        .map(|result| result.output)
}
