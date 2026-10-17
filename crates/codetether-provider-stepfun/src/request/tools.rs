//! Function tool schemas.
use crate::*;
impl StepFunProvider {
    pub(crate) fn convert_tools(&self, tools: &[ToolDefinition]) -> Vec<ChatTool> {
        tools
            .iter()
            .map(|t| ChatTool {
                r#type: "function".to_string(),
                function: ChatFunction {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                },
            })
            .collect()
    }
}
