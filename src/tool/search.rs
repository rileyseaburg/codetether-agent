//! Search tools: grep

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use ignore::WalkBuilder;
use regex::Regex;
use serde_json::{Value, json};
use std::time::{Duration, Instant};

const DEFAULT_GREP_LIMIT: usize = 50;
const MAX_GREP_LIMIT: usize = 500;
const DEFAULT_GREP_TIMEOUT_SECS: u64 = 15;
const MAX_GREP_TIMEOUT_SECS: u64 = 120;
const DEFAULT_GREP_MAX_SCANNED_FILES: usize = 10_000;
const DEFAULT_GREP_MAX_FILE_BYTES: u64 = 1024 * 1024;

/// Search for text in files
pub struct GrepTool;

impl Default for GrepTool {
    fn default() -> Self {
        Self::new()
    }
}

impl GrepTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for GrepTool {
    fn id(&self) -> &str {
        "grep"
    }

    fn name(&self) -> &str {
        "Grep Search"
    }

    fn description(&self) -> &str {
        "grep(pattern: string, path?: string, is_regex?: bool, include?: string, limit?: int) - Search for text or regex patterns in files. Respects .gitignore by default."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {
                    "type": "string",
                    "description": "The text or regex pattern to search for"
                },
                "path": {
                    "type": "string",
                    "description": "Directory or file to search in (default: current directory)"
                },
                "is_regex": {
                    "type": "boolean",
                    "description": "Whether the pattern is a regex (default: false)"
                },
                "include": {
                    "type": "string",
                    "description": "Glob pattern to include files (e.g., *.rs)"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of matches to return"
                },
                "timeout_secs": {
                    "type": "integer",
                    "description": "Maximum search time in seconds before returning partial results"
                }
            },
            "required": ["pattern"],
            "example": {
                "pattern": "fn main",
                "path": "src/",
                "include": "*.rs"
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let pattern = match args["pattern"].as_str() {
            Some(p) => p,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "grep",
                    "pattern is required",
                    Some(vec!["pattern"]),
                    Some(json!({"pattern": "search text", "path": "src/"})),
                ));
            }
        };
        let search_path = args["path"].as_str().unwrap_or(".");
        let is_regex = args["is_regex"].as_bool().unwrap_or(false);
        let include = args["include"].as_str();
        let limit = args["limit"]
            .as_u64()
            .map(|n| n as usize)
            .unwrap_or(DEFAULT_GREP_LIMIT)
            .clamp(1, MAX_GREP_LIMIT);
        let timeout_secs = args["timeout_secs"]
            .as_u64()
            .or_else(|| env_u64("CODETETHER_GREP_TIMEOUT_SECS"))
            .unwrap_or(DEFAULT_GREP_TIMEOUT_SECS)
            .clamp(1, MAX_GREP_TIMEOUT_SECS);
        let max_scanned_files = env_usize("CODETETHER_GREP_MAX_SCANNED_FILES")
            .unwrap_or(DEFAULT_GREP_MAX_SCANNED_FILES)
            .max(1);
        let max_file_bytes = env_u64("CODETETHER_GREP_MAX_FILE_BYTES")
            .unwrap_or(DEFAULT_GREP_MAX_FILE_BYTES)
            .max(1);

        let regex = if is_regex {
            Regex::new(pattern)?
        } else {
            Regex::new(&regex::escape(pattern))?
        };
        let include_pattern = include.and_then(|pattern| glob::Pattern::new(pattern).ok());

        let started = Instant::now();
        let deadline = started + Duration::from_secs(timeout_secs);
        let mut results = Vec::new();
        let mut scanned_files = 0usize;
        let mut skipped_oversize = 0usize;
        let mut skipped_unreadable = 0usize;
        let mut timed_out = false;
        let mut scan_limit_reached = false;
        let mut walker = WalkBuilder::new(search_path);
        walker.hidden(false).git_ignore(true);

        for entry in walker.build() {
            if Instant::now() >= deadline {
                timed_out = true;
                break;
            }

            if results.len() >= limit {
                break;
            }

            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }

            let path = entry.path();

            // Check include pattern
            if let Some(include_pattern) = &include_pattern
                && !include_pattern.matches_path(path)
            {
                continue;
            }

            scanned_files += 1;
            if scanned_files > max_scanned_files {
                scan_limit_reached = true;
                break;
            }

            let Some(remaining) = remaining_duration(deadline) else {
                timed_out = true;
                break;
            };
            let metadata = match tokio::time::timeout(remaining, tokio::fs::metadata(path)).await {
                Ok(Ok(metadata)) => metadata,
                Ok(Err(_)) => {
                    skipped_unreadable += 1;
                    continue;
                }
                Err(_) => {
                    timed_out = true;
                    break;
                }
            };
            if metadata.len() > max_file_bytes {
                skipped_oversize += 1;
                continue;
            }

            // Read and search file
            let Some(remaining) = remaining_duration(deadline) else {
                timed_out = true;
                break;
            };
            let content =
                match tokio::time::timeout(remaining, tokio::fs::read_to_string(path)).await {
                    Ok(Ok(content)) => content,
                    Ok(Err(_)) => {
                        skipped_unreadable += 1;
                        continue;
                    }
                    Err(_) => {
                        timed_out = true;
                        break;
                    }
                };

            for (line_num, line) in content.lines().enumerate() {
                if results.len() >= limit {
                    break;
                }

                if regex.is_match(line) {
                    results.push(format!(
                        "{}:{}: {}",
                        path.display(),
                        line_num + 1,
                        line.trim()
                    ));
                }
            }
        }

        let result_limit_reached = results.len() >= limit;
        let truncated = result_limit_reached || timed_out || scan_limit_reached;
        let mut output = results.join("\n");
        if output.is_empty() {
            output = "No matches found".to_string();
        }
        if timed_out {
            output.push_str(&format!(
                "\n[grep stopped after {timeout_secs}s; scanned {scanned_files} files. Narrow path/include or raise timeout_secs.]"
            ));
        } else if scan_limit_reached {
            output.push_str(&format!(
                "\n[grep stopped after scanning {max_scanned_files} files. Narrow path/include.]"
            ));
        }

        let result = if timed_out || scan_limit_reached {
            ToolResult::error(output)
        } else {
            ToolResult::success(output)
        };

        Ok(result
            .with_metadata("count", json!(results.len()))
            .with_metadata("truncated", json!(truncated))
            .with_metadata("scanned_files", json!(scanned_files))
            .with_metadata("skipped_oversize", json!(skipped_oversize))
            .with_metadata("skipped_unreadable", json!(skipped_unreadable))
            .with_metadata("timed_out", json!(timed_out))
            .with_metadata("scan_limit_reached", json!(scan_limit_reached)))
    }
}

fn env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.parse().ok()
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.parse().ok()
}

fn remaining_duration(deadline: Instant) -> Option<Duration> {
    deadline.checked_duration_since(Instant::now())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn grep_honors_result_limit() {
        let dir = tempfile::tempdir().expect("tempdir");
        tokio::fs::write(dir.path().join("a.txt"), "needle one\nneedle two\n")
            .await
            .expect("write fixture");
        tokio::fs::write(dir.path().join("b.txt"), "needle three\n")
            .await
            .expect("write fixture");

        let result = GrepTool::new()
            .execute(json!({
                "pattern": "needle",
                "path": dir.path().to_string_lossy(),
                "limit": 1
            }))
            .await
            .expect("grep executes");

        assert!(result.success);
        assert_eq!(result.metadata["count"], json!(1));
        assert_eq!(result.metadata["truncated"], json!(true));
        assert!(result.output.contains("needle"));
    }

    #[tokio::test]
    async fn grep_skips_oversized_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut file = tokio::fs::File::create(dir.path().join("large.txt"))
            .await
            .expect("create fixture");
        file.write_all(&vec![b'x'; (DEFAULT_GREP_MAX_FILE_BYTES + 1) as usize])
            .await
            .expect("write fixture");
        file.flush().await.expect("flush fixture");

        let result = GrepTool::new()
            .execute(json!({
                "pattern": "needle",
                "path": dir.path().to_string_lossy()
            }))
            .await
            .expect("grep executes");

        assert!(result.success);
        assert_eq!(result.metadata["count"], json!(0));
        assert_eq!(result.metadata["skipped_oversize"], json!(1));
        assert!(result.output.contains("No matches found"));
    }
}
