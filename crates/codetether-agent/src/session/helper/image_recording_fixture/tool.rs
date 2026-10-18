//! Read-only mock returns long prose and two separate canonical image entries.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) const IMAGES: [(&str, &str); 2] = [
    ("data:image/png;base64,AQID", "image/png"),
    ("data:image/jpeg;base64,BAUG", "image/jpeg"),
];

#[derive(Default)]
pub(in crate::session::helper) struct ImageTool(pub AtomicUsize);

#[async_trait]
impl Tool for ImageTool {
    fn id(&self) -> &str {
        "read"
    }

    fn name(&self) -> &str {
        self.id()
    }

    fn description(&self) -> &str {
        "Mock image-producing read"
    }

    fn parameters(&self) -> Value {
        json!({"type": "object"})
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        self.0.fetch_add(1, Ordering::SeqCst);
        let mut result = ToolResult::success("image evidence\n".repeat(1500));
        result.success = input["success"].as_bool().expect("fixture status");
        Ok(result.with_metadata(
            "image_data_url",
            json!(IMAGES.map(
                |(data_url, mime_type)| json!({"data_url": data_url, "mime_type": mime_type})
            )),
        ))
    }
}