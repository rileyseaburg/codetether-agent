//! Shared tests helpers for apply_patch.

use super::ApplyPatchTool;
use crate::tool::{Tool, ToolResult};
use serde_json::Value;
use std::path::Path;

pub(super) const ORIGINAL: &str = "line1\nold line\nline3";

pub(super) fn sample_patch() -> &'static str {
    "--- a/file.txt\n+++ b/file.txt\n@@ -1,3 +1,3 @@\n line1\n-old line\n+new line\n line3"
}

pub(super) fn seed(root: &Path) {
    std::fs::write(root.join("file.txt"), ORIGINAL).expect("write fixture");
}

pub(super) async fn execute(root: &Path, params: Value) -> ToolResult {
    ApplyPatchTool::with_root(root.to_path_buf())
        .execute(params)
        .await
        .expect("execute patch tool")
}

pub(super) struct EnvGuard(&'static str);

impl EnvGuard {
    pub(super) fn set(key: &'static str, value: &str) -> Self {
        unsafe { std::env::set_var(key, value) };
        Self(key)
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var(self.0) };
    }
}
