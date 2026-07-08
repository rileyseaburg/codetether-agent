//! [`StepVars`] struct for the step-model restore helper.

use std::path::PathBuf;
use std::sync::Arc;

use crate::provider::{Provider, ToolDefinition};
use crate::session::Session;
use crate::session::helper::request_state::ProviderStepState;
use crate::tool::ToolRegistry;

/// All mutable step-loop state that may need restoring at each step start.
pub(crate) struct StepVars<'a> {
    pub selected_provider: &'a mut String,
    pub model: &'a mut String,
    pub provider: &'a mut Arc<dyn Provider>,
    pub provider_state: &'a mut ProviderStepState,
    pub tool_registry: &'a mut Arc<ToolRegistry>,
    pub tool_definitions: &'a mut Vec<ToolDefinition>,
    pub advertised_tool_definitions: &'a mut Vec<ToolDefinition>,
    pub temperature: &'a mut Option<f32>,
    pub model_supports_tools: &'a mut bool,
    pub system_prompt: &'a mut String,
    pub session: &'a mut Session,
    pub cwd: &'a PathBuf,
}
