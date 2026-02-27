//! Code Search Tool - Search code in the workspace using ripgrep-style patterns.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::PathBuf;
use walkdir::WalkDir;

const MAX_RESULTS: usize = 50;
const MAX_CONTEXT_LINES: usize = 3;

pub struct CodeSearchTool {
    root: PathBuf,
}

impl Default for CodeSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl CodeSearchTool {
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    fn should_skip(&self, path: &std::path::Path) -> bool {
        let skip_dirs = [
            ".git",
            "node_modules",
            "target",
            "dist",
            ".next",
            "__pycache__",
            ".venv",
            "vendor",
        ];
        path.components()
            .any(|c| skip_dirs.contains(&c.as_os_str().to_str().unwrap_or("")))
    }

    fn is_text_file(&self, path: &std::path::Path) -> bool {
        let text_exts = [
            "rs", "ts", "js", "tsx", "jsx", "py", "go", "java", "c", "cpp", "h", "hpp", "md",
            "txt", "json", "yaml", "yml", "toml", "sh", "bash", "zsh", "html", "css", "scss",
        ];
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| text_exts.contains(&e))
            .unwrap_or(false)
    }

    fn search_file(
        &self,
        path: &std::path::Path,
        pattern: &regex::Regex,
        context: usize,
    ) -> Result<Vec<Match>> {
        let content = std::fs::read_to_string(path)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut matches = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            if pattern.is_match(line) {
                let start = idx.saturating_sub(context);
                let end = (idx + context + 1).min(lines.len());
                let context_lines: Vec<String> = lines[start..end]
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        let line_num = start + i + 1;
                        let marker = if start + i == idx { ">" } else { " " };
                        format!("{} {:4}: {}", marker, line_num, l)
                    })
                    .collect();

                matches.push(Match {
                    path: path
                        .strip_prefix(&self.root)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .to_string(),
                    line: idx + 1,
                    content: line.to_string(),
                    context: context_lines.join("\n"),
                });
            }
        }
        Ok(matches)
    }
}

#[derive(Debug)]
struct Match {
    path: String,
    line: usize,
    #[allow(dead_code)]
    content: String,
    context: String,
}

#[derive(Deserialize)]
struct Params {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    file_pattern: Option<String>,
    #[serde(default = "default_context")]
    context_lines: usize,
    #[serde(default)]
    case_sensitive: bool,
}

fn default_context() -> usize {
    2
}

#[async_trait]
impl Tool for CodeSearchTool {
    fn id(&self) -> &str {
        "codesearch"
    }
    fn name(&self) -> &str {
        "Code Search"
    }
    fn description(&self) -> &str {
        "Search for code patterns in the workspace. Supports regex."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {"type": "string", "description": "Search pattern (regex supported)"},
                "path": {"type": "string", "description": "Subdirectory to search in"},
                "file_pattern": {"type": "string", "description": "Glob pattern for files (e.g., *.rs)"},
                "context_lines": {"type": "integer", "default": 2, "description": "Lines of context"},
                "case_sensitive": {"type": "boolean", "default": false}
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        let regex = regex::RegexBuilder::new(&p.pattern)
            .case_insensitive(!p.case_sensitive)
            .build()
            .context("Invalid regex pattern")?;

        let search_root = match &p.path {
            Some(subpath) => self.root.join(subpath),
            None => self.root.clone(),
        };

        let file_glob = p
            .file_pattern
            .as_ref()
            .and_then(|pat| glob::Pattern::new(pat).ok());

        let mut all_matches = Vec::new();

        for entry in WalkDir::new(&search_root)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() || self.should_skip(path) || !self.is_text_file(path) {
                continue;
            }

            if let Some(ref glob) = file_glob
                && !glob.matches_path(path)
            {
                continue;
            }

            if let Ok(matches) =
                self.search_file(path, &regex, p.context_lines.min(MAX_CONTEXT_LINES))
            {
                all_matches.extend(matches);
                if all_matches.len() >= MAX_RESULTS {
                    break;
                }
            }
        }

        if all_matches.is_empty() {
            return Ok(ToolResult::success(format!(
                "No matches found for pattern: {}",
                p.pattern
            )));
        }

        let output = all_matches
            .iter()
            .take(MAX_RESULTS)
            .map(|m| format!("{}:{}\n{}", m.path, m.line, m.context))
            .collect::<Vec<_>>()
            .join("\n\n");

        let truncated = all_matches.len() > MAX_RESULTS;
        let msg = if truncated {
            format!(
                "Found {} matches (showing first {}):\n\n{}",
                all_matches.len(),
                MAX_RESULTS,
                output
            )
        } else {
            format!("Found {} matches:\n\n{}", all_matches.len(), output)
        };

        Ok(ToolResult::success(msg).with_metadata("match_count", json!(all_matches.len())))
    }
}
