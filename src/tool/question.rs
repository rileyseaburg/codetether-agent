//! Async question-tool adapter. Selection and rendezvous live in the question crate.

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use codetether_question::{Request, runtime};
use serde_json::{Value, json};

/// Ask batched questions through an attached interactive frontend.
///
/// # Examples
/// ```
/// use codetether_agent::tool::{question::QuestionTool, Tool};
/// assert_eq!(QuestionTool::new().id(), "question");
/// ```
pub struct QuestionTool;

impl Default for QuestionTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "question_tests.rs"]
mod tests;

impl QuestionTool {
    /// Construct the stateless runtime adapter.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for QuestionTool {
    fn id(&self) -> &str {
        "question"
    }
    fn name(&self) -> &str {
        "Ask Question"
    }
    fn description(&self) -> &str {
        "Ask the user questions with labeled choices, descriptions, multiple selection, and custom answers. Put recommended choices first. Custom answers are offered automatically."
    }
    fn parameters(&self) -> Value {
        codetether_question::parameters()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let request = match Request::parse(params.clone()) {
            Ok(request) => request,
            Err(error) => return Ok(ToolResult::error(error.to_string())),
        };
        let session = params.get("__ct_session_id").and_then(Value::as_str);
        match runtime::ask(request, session).await {
            Ok(answers) => Ok(ToolResult::success(json!({"answers": answers}).to_string())
                .with_metadata("answers", json!(answers))),
            Err(error) => Ok(ToolResult::error(error.to_string())),
        }
    }
}
