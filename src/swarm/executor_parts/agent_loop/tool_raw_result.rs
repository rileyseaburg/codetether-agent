//! Outcome of one raw tool invocation.

use crate::{provider::ContentPart, tool::ToolResult};

pub(super) struct Result {
    pub output: String,
    pub images: Vec<ContentPart>,
    pub success: bool,
    pub timed_out: bool,
}

impl Result {
    pub(super) fn from_tool(result: ToolResult) -> Self {
        Self {
            images: crate::tool::result_images::content(Some(&result.metadata)),
            output: if result.success {
                result.output
            } else {
                format!("Tool error: {}", result.output)
            },
            success: result.success,
            timed_out: false,
        }
    }

    pub(super) fn failure(output: String, timed_out: bool) -> Self {
        Self {
            output,
            images: Vec::new(),
            success: false,
            timed_out,
        }
    }
}
