//! Multi-Edit Tool
//!
//! Apply multiple file edits atomically. Validates all edits first, then
//! writes all changes only if every edit passes validation.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

use super::{Tool, ToolResult};

pub struct MultiEditTool;

impl Default for MultiEditTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiEditTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for MultiEditTool {
    fn id(&self) -> &str {
        "multiedit"
    }

    fn name(&self) -> &str {
        "Multi Edit"
    }

    fn description(&self) -> &str {
        "Apply multiple file edits atomically. Validates all edits, then writes all changes. \
         If any edit fails validation, no files are modified. Each edit replaces old_string \
         with new_string in the given file."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "edits": {
                    "type": "array",
                    "description": "Array of edit operations to apply atomically",
                    "items": {
                        "type": "object",
                        "properties": {
                            "file": {
                                "type": "string",
                                "description": "Path to the file to edit"
                            },
                            "old_string": {
                                "type": "string",
                                "description": "The exact string to find and replace (must appear exactly once)"
                            },
                            "new_string": {
                                "type": "string",
                                "description": "The replacement string"
                            }
                        },
                        "required": ["file", "old_string", "new_string"]
                    }
                }
            },
            "required": ["edits"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        // ── Parse with detailed error messages ──────────────────────
        let edits_val = match params.get("edits") {
            Some(v) => v,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "multiedit",
                    "Missing required field 'edits'. Provide an array of {file, old_string, new_string} objects.",
                    Some(vec!["edits"]),
                    Some(json!({
                        "edits": [
                            {"file": "src/main.rs", "old_string": "old code", "new_string": "new code"}
                        ]
                    })),
                ));
            }
        };

        let edits_arr = match edits_val.as_array() {
            Some(a) => a,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "multiedit",
                    "'edits' must be an array, not a single object.",
                    Some(vec!["edits"]),
                    Some(json!({
                        "edits": [
                            {"file": "src/main.rs", "old_string": "old code", "new_string": "new code"}
                        ]
                    })),
                ));
            }
        };

        if edits_arr.is_empty() {
            return Ok(ToolResult::error(
                "No edits provided. 'edits' array is empty.",
            ));
        }

        // ── Extract and validate each edit entry ────────────────────
        struct EditOp {
            file: String,
            old_string: String,
            new_string: String,
        }

        let mut ops: Vec<EditOp> = Vec::with_capacity(edits_arr.len());
        for (i, entry) in edits_arr.iter().enumerate() {
            let file = match entry.get("file").and_then(|v| v.as_str()) {
                Some(f) => f.to_string(),
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_ARGUMENT",
                        "multiedit",
                        &format!("edits[{i}]: missing or non-string 'file' field"),
                        Some(vec!["edits", "file"]),
                        Some(
                            json!({"file": "src/example.rs", "old_string": "...", "new_string": "..."}),
                        ),
                    ));
                }
            };
            let old_string = match entry.get("old_string").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_ARGUMENT",
                        "multiedit",
                        &format!("edits[{i}] ({file}): missing or non-string 'old_string' field"),
                        Some(vec!["edits", "old_string"]),
                        Some(
                            json!({"file": file, "old_string": "exact text to find", "new_string": "replacement"}),
                        ),
                    ));
                }
            };
            let new_string = match entry.get("new_string").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_ARGUMENT",
                        "multiedit",
                        &format!("edits[{i}] ({file}): missing or non-string 'new_string' field"),
                        Some(vec!["edits", "new_string"]),
                        Some(
                            json!({"file": file, "old_string": old_string, "new_string": "replacement text"}),
                        ),
                    ));
                }
            };
            ops.push(EditOp {
                file,
                old_string,
                new_string,
            });
        }

        // ── Phase 1: Validate all edits (no writes yet) ────────────
        // Each validated edit becomes (path, original_content, new_content).
        let mut validated: Vec<(PathBuf, String, String)> = Vec::with_capacity(ops.len());
        let mut errors: Vec<String> = Vec::new();

        // When multiple edits target the same file, we must chain them
        // on the accumulated content rather than re-reading from disk.
        let mut content_cache: HashMap<PathBuf, String> = HashMap::new();

        for (i, op) in ops.iter().enumerate() {
            let path = PathBuf::from(&op.file);

            // Read or reuse cached content
            let content = if let Some(cached) = content_cache.get(&path) {
                cached.clone()
            } else if path.exists() {
                match fs::read_to_string(&path).await {
                    Ok(c) => c,
                    Err(e) => {
                        errors.push(format!("edits[{i}] {}: cannot read file: {e}", op.file));
                        continue;
                    }
                }
            } else {
                errors.push(format!("edits[{i}] {}: file does not exist", op.file));
                continue;
            };

            let count = content.matches(&op.old_string).count();
            if count == 0 {
                let preview: String = op.old_string.chars().take(80).collect();
                errors.push(format!(
                    "edits[{i}] {}: old_string not found. First 80 chars: \"{}\"",
                    op.file, preview
                ));
                continue;
            }
            if count > 1 {
                errors.push(format!(
                    "edits[{i}] {}: old_string found {count} times (must be unique). Add more context.",
                    op.file
                ));
                continue;
            }

            let new_content = content.replacen(&op.old_string, &op.new_string, 1);
            content_cache.insert(path.clone(), new_content.clone());
            validated.push((path, content, new_content));
        }

        if !errors.is_empty() {
            let error_list = errors.join("\n");
            return Ok(ToolResult {
                output: format!(
                    "Validation failed for {} of {} edits. No files were modified.\n\n{error_list}",
                    errors.len(),
                    ops.len()
                ),
                success: false,
                metadata: HashMap::new(),
            });
        }

        // ── Phase 2: Write all changes ──────────────────────────────
        // Deduplicate by path — use the final accumulated content from
        // the cache (handles multiple edits to the same file).
        let mut written: HashMap<PathBuf, bool> = HashMap::new();
        let mut write_errors: Vec<String> = Vec::new();

        for (path, _original, _new) in &validated {
            if written.contains_key(path) {
                continue;
            }
            let final_content = content_cache.get(path).unwrap();
            match fs::write(path, final_content).await {
                Ok(()) => {
                    written.insert(path.clone(), true);
                }
                Err(e) => {
                    write_errors.push(format!("{}: write failed: {e}", path.display()));
                    written.insert(path.clone(), false);
                }
            }
        }

        if !write_errors.is_empty() {
            return Ok(ToolResult {
                output: format!(
                    "Write errors (some files may have been partially updated):\n{}",
                    write_errors.join("\n")
                ),
                success: false,
                metadata: HashMap::new(),
            });
        }

        // ── Build summary ───────────────────────────────────────────
        let unique_files = written.len();
        let total_edits = ops.len();
        let mut summary_lines: Vec<String> = Vec::new();
        for (path, original, new_content) in &validated {
            let old_lines = original.lines().count();
            let new_lines = new_content.lines().count();
            let delta = new_lines as i64 - old_lines as i64;
            let sign = if delta >= 0 { "+" } else { "" };
            summary_lines.push(format!("✓ {} ({sign}{delta} lines)", path.display()));
        }

        Ok(ToolResult::success(format!(
            "Applied {total_edits} edit(s) across {unique_files} file(s):\n{}",
            summary_lines.join("\n")
        )))
    }
}
