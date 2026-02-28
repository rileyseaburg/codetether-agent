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

fn apply_exact_replace(
    content: &str,
    old_string: &str,
    new_string: &str,
    file: &str,
    index: usize,
) -> std::result::Result<String, String> {
    let count = content.matches(old_string).count();
    if count == 0 {
        let preview: String = old_string.chars().take(80).collect();
        return Err(format!(
            "edits[{index}] {file}: old_string not found. First 80 chars: \"{preview}\""
        ));
    }
    if count > 1 {
        return Err(format!(
            "edits[{index}] {file}: old_string found {count} times (must be unique). Add more context."
        ));
    }
    Ok(content.replacen(old_string, new_string, 1))
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
         with new_string in the given file. Uses Morph backend by default (disable with \
         CODETETHER_MORPH_TOOL_BACKEND=0); instruction/update can guide Morph behavior per edit."
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
                            },
                            "instruction": {
                                "type": "string",
                                "description": "Optional Morph instruction for this edit."
                            },
                            "update": {
                                "type": "string",
                                "description": "Optional Morph update snippet for this edit."
                            }
                        },
                        "required": ["file"]
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
            old_string: Option<String>,
            new_string: Option<String>,
            instruction: Option<String>,
            update: Option<String>,
            use_morph: bool,
        }

        let mut ops: Vec<EditOp> = Vec::with_capacity(edits_arr.len());
        let morph_enabled = super::morph_backend::should_use_morph_backend();
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
            let old_string = entry
                .get("old_string")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let new_string = entry
                .get("new_string")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let instruction = entry
                .get("instruction")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let update = entry
                .get("update")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            let has_replacement_pair = old_string.is_some() && new_string.is_some();
            let use_morph = morph_enabled
                && (instruction.is_some() || update.is_some() || has_replacement_pair);

            if !use_morph && (old_string.is_none() || new_string.is_none()) {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "multiedit",
                    &format!(
                        "edits[{i}] ({file}): provide old_string/new_string, or enable Morph backend and provide instruction/update"
                    ),
                    Some(vec!["edits"]),
                    Some(json!({
                        "file": file,
                        "old_string": "exact text to find",
                        "new_string": "replacement text",
                        "instruction": "Optional Morph instruction",
                        "update": "Optional Morph update snippet"
                    })),
                ));
            }

            ops.push(EditOp {
                file,
                old_string,
                new_string,
                instruction,
                update,
                use_morph,
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

            let new_content = if op.use_morph {
                let inferred_instruction = op
                    .instruction
                    .clone()
                    .or_else(|| {
                        op.old_string
                            .as_deref()
                            .zip(op.new_string.as_deref())
                            .map(|(old, new)| {
                                format!(
                                    "Replace the target snippet exactly once while preserving behavior.\nOld snippet:\n{old}\n\nNew snippet:\n{new}"
                                )
                            })
                    })
                    .unwrap_or_else(|| {
                        "Apply the requested update precisely and return only the updated file."
                            .to_string()
                    });
                let inferred_update = op
                    .update
                    .clone()
                    .or_else(|| {
                        op.old_string
                            .as_deref()
                            .zip(op.new_string.as_deref())
                            .map(|(old, new)| {
                                format!(
                                    "// Replace this snippet:\n{old}\n// With this snippet:\n{new}\n// ...existing code..."
                                )
                            })
                    })
                    .unwrap_or_else(|| "// ...existing code...".to_string());

                match super::morph_backend::apply_edit_with_morph(
                    &content,
                    &inferred_instruction,
                    &inferred_update,
                )
                .await
                {
                    Ok(c) => c,
                    Err(e) => {
                        if let (Some(old_string), Some(new_string)) =
                            (op.old_string.as_deref(), op.new_string.as_deref())
                        {
                            tracing::warn!(
                                file = %op.file,
                                error = %e,
                                "Morph backend failed for multiedit op; falling back to exact replacement"
                            );
                            match apply_exact_replace(&content, old_string, new_string, &op.file, i)
                            {
                                Ok(c) => c,
                                Err(msg) => {
                                    errors.push(msg);
                                    continue;
                                }
                            }
                        } else {
                            errors
                                .push(format!("edits[{i}] {}: Morph backend failed: {e}", op.file));
                            continue;
                        }
                    }
                }
            } else {
                let old_string = op.old_string.as_deref().unwrap_or_default();
                let new_string = op.new_string.as_deref().unwrap_or_default();
                match apply_exact_replace(&content, old_string, new_string, &op.file, i) {
                    Ok(c) => c,
                    Err(msg) => {
                        errors.push(msg);
                        continue;
                    }
                }
            };

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
