//! Interactive question tool backed by the session-scoped question runtime.
//!
//! Construct [QuestionTool] and invoke it through the shared Tool contract.
//! Requests, frontend rendezvous and selection live in codetether-question.
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
mod execute;
mod schema;
mod tool;
use codetether_question::{Request, runtime};
mod construct;
#[cfg(test)]
mod tests;

/// Ask batched questions through an attached interactive frontend.
///
/// # Examples
/// ```
/// use codetether_question_tool::QuestionTool;
/// use codetether_tool_core::Tool;
/// assert_eq!(QuestionTool::new().id(), "question");
/// ```
pub struct QuestionTool;
