//! Question Tool - Ask the user a question and wait for response.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use super::{Tool, ToolResult};
use std::io::{self, Write};

pub struct QuestionTool;

impl Default for QuestionTool {
    fn default() -> Self { Self::new() }
}

impl QuestionTool {
    pub fn new() -> Self { Self }
}

#[derive(Deserialize)]
struct Params {
    question: String,
    #[serde(default)]
    options: Vec<String>,
    #[serde(default)]
    default: Option<String>,
}

#[async_trait]
impl Tool for QuestionTool {
    fn id(&self) -> &str { "question" }
    fn name(&self) -> &str { "Ask Question" }
    fn description(&self) -> &str { "Ask the user a question and wait for their response. Use for clarification or user input." }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "question": {"type": "string", "description": "Question to ask the user"},
                "options": {"type": "array", "items": {"type": "string"}, "description": "Optional list of valid responses"},
                "default": {"type": "string", "description": "Default value if user presses Enter"}
            },
            "required": ["question"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;
        
        // Print question
        print!("\n📝 {}", p.question);
        
        if !p.options.is_empty() {
            print!(" [{}]", p.options.join("/"));
        }
        
        if let Some(ref default) = p.default {
            print!(" (default: {})", default);
        }
        
        print!(": ");
        io::stdout().flush()?;
        
        // Read response
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let response = input.trim().to_string();
        
        let final_response = if response.is_empty() {
            p.default.unwrap_or_default()
        } else {
            response
        };
        
        // Validate against options if provided
        if !p.options.is_empty() && !p.options.contains(&final_response) {
            return Ok(ToolResult::error(format!(
                "Invalid response '{}'. Valid options: {}", 
                final_response, 
                p.options.join(", ")
            )));
        }
        
        Ok(ToolResult::success(final_response.clone())
            .with_metadata("response", json!(final_response)))
    }
}
