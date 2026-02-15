//! Apply Patch Tool - Apply unified diff patches to files.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::path::PathBuf;

pub struct ApplyPatchTool {
    root: PathBuf,
}

impl Default for ApplyPatchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplyPatchTool {
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    #[allow(dead_code)]
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    fn parse_patch(&self, patch: &str) -> Result<Vec<PatchHunk>> {
        let mut hunks = Vec::new();
        let mut current_file: Option<String> = None;
        let mut current_hunk: Option<HunkBuilder> = None;

        for line in patch.lines() {
            if line.starts_with("--- ") {
                // Old file header, ignore for now
            } else if line.starts_with("+++ ") {
                // New file header
                let path = line.strip_prefix("+++ ").unwrap_or("");
                let path = path.strip_prefix("b/").unwrap_or(path);
                let path = path.split('\t').next().unwrap_or(path);
                current_file = Some(path.to_string());
            } else if line.starts_with("@@ ") {
                // Hunk header: @@ -start,count +start,count @@
                if let Some(hunk) = current_hunk.take() {
                    if let Some(file) = &current_file {
                        hunks.push(hunk.build(file.clone()));
                    }
                }

                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let old_range = parts[1].strip_prefix('-').unwrap_or(parts[1]);
                    let old_start: usize = old_range
                        .split(',')
                        .next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1);

                    current_hunk = Some(HunkBuilder {
                        start_line: old_start,
                        old_lines: Vec::new(),
                        new_lines: Vec::new(),
                    });
                }
            } else if let Some(ref mut hunk) = current_hunk {
                if let Some(stripped) = line.strip_prefix('-') {
                    hunk.old_lines.push(stripped.to_string());
                } else if let Some(stripped) = line.strip_prefix('+') {
                    hunk.new_lines.push(stripped.to_string());
                } else if line.starts_with(' ') || line.is_empty() {
                    let content = if line.is_empty() { "" } else { &line[1..] };
                    hunk.old_lines.push(content.to_string());
                    hunk.new_lines.push(content.to_string());
                }
            }
        }

        // Finalize last hunk
        if let Some(hunk) = current_hunk {
            if let Some(file) = &current_file {
                hunks.push(hunk.build(file.clone()));
            }
        }

        Ok(hunks)
    }

    fn apply_hunk(&self, content: &str, hunk: &PatchHunk) -> Result<String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();

        // Find matching location (fuzzy match)
        let mut match_start = None;
        for i in 0..=lines.len().saturating_sub(hunk.old_lines.len()) {
            let mut matches = true;
            for (j, old_line) in hunk.old_lines.iter().enumerate() {
                if i + j >= lines.len() || lines[i + j].trim() != old_line.trim() {
                    matches = false;
                    break;
                }
            }
            if matches {
                match_start = Some(i);
                break;
            }
        }

        let match_start =
            match_start.ok_or_else(|| anyhow::anyhow!("Could not find hunk location"))?;

        // Build result
        result.extend(lines[..match_start].iter().map(|s| s.to_string()));
        result.extend(hunk.new_lines.clone());
        result.extend(
            lines[match_start + hunk.old_lines.len()..]
                .iter()
                .map(|s| s.to_string()),
        );

        Ok(result.join("\n"))
    }
}

struct HunkBuilder {
    start_line: usize,
    old_lines: Vec<String>,
    new_lines: Vec<String>,
}

impl HunkBuilder {
    fn build(self, file: String) -> PatchHunk {
        PatchHunk {
            file,
            start_line: self.start_line,
            old_lines: self.old_lines,
            new_lines: self.new_lines,
        }
    }
}

#[derive(Debug)]
struct PatchHunk {
    file: String,
    start_line: usize,
    old_lines: Vec<String>,
    new_lines: Vec<String>,
}

#[async_trait]
impl Tool for ApplyPatchTool {
    fn id(&self) -> &str {
        "apply_patch"
    }
    fn name(&self) -> &str {
        "Apply Patch"
    }
    fn description(&self) -> &str {
        "Apply a unified diff patch to files in the workspace."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "patch": {"type": "string", "description": "Unified diff patch content"},
                "dry_run": {"type": "boolean", "default": false, "description": "Preview without applying"}
            },
            "required": ["patch"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let patch = match params.get("patch").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "apply_patch",
                    "patch is required and must be a non-empty string containing a unified diff",
                    Some(vec!["patch"]),
                    Some(json!({
                        "patch": "--- a/file.rs\n+++ b/file.rs\n@@ -1,3 +1,3 @@\n line1\n-old line\n+new line\n line3",
                        "dry_run": false
                    })),
                ));
            }
        };
        let dry_run = params
            .get("dry_run")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let hunks = self.parse_patch(&patch)?;

        if hunks.is_empty() {
            return Ok(ToolResult::structured_error(
                "PARSE_ERROR",
                "apply_patch",
                "No valid hunks found in patch. Make sure the patch is in unified diff format with proper --- a/, +++ b/, and @@ headers.",
                None,
                Some(json!({
                    "expected_format": "--- a/path/to/file\n+++ b/path/to/file\n@@ -start,count +start,count @@\n context line\n-removed line\n+added line\n context line",
                    "hint": "Lines starting with - are removed, + are added, space are context"
                })),
            ));
        }

        let mut results = Vec::new();
        let mut files_modified = Vec::new();

        // Group hunks by file
        let mut by_file: std::collections::HashMap<String, Vec<&PatchHunk>> =
            std::collections::HashMap::new();
        for hunk in &hunks {
            by_file.entry(hunk.file.clone()).or_default().push(hunk);
        }

        for (file, file_hunks) in by_file {
            let path = self.root.join(&file);

            let mut content = if path.exists() {
                std::fs::read_to_string(&path).context(format!("Failed to read {}", file))?
            } else {
                String::new()
            };

            for hunk in file_hunks {
                match self.apply_hunk(&content, hunk) {
                    Ok(new_content) => {
                        content = new_content;
                        results.push(format!(
                            "✓ Applied hunk to {} at line {}",
                            file, hunk.start_line
                        ));
                    }
                    Err(e) => {
                        results.push(format!("✗ Failed to apply hunk to {}: {}", file, e));
                    }
                }
            }

            if !dry_run {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&path, &content)?;
                files_modified.push(file);
            }
        }

        let action = if dry_run { "Would modify" } else { "Modified" };
        let summary = format!(
            "{} {} files:\n{}",
            action,
            files_modified.len(),
            results.join("\n")
        );

        Ok(ToolResult::success(summary).with_metadata("files", json!(files_modified)))
    }
}
