//! Additional file tools: tree, fileinfo, headtail, diff

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::path::Path;
use tokio::fs;

/// Tree view of directory structure
pub struct TreeTool;

impl TreeTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for TreeTool {
    fn id(&self) -> &str {
        "tree"
    }

    fn name(&self) -> &str {
        "Directory Tree"
    }

    fn description(&self) -> &str {
        "tree(path: string, depth?: int, show_hidden?: bool, show_size?: bool) - Display a tree view of directory structure. Great for understanding project layout."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The root directory to display"
                },
                "depth": {
                    "type": "integer",
                    "description": "Maximum depth to traverse (default: 3)",
                    "default": 3
                },
                "show_hidden": {
                    "type": "boolean",
                    "description": "Show hidden files (default: false)",
                    "default": false
                },
                "show_size": {
                    "type": "boolean",
                    "description": "Show file sizes (default: false)",
                    "default": false
                },
                "gitignore": {
                    "type": "boolean",
                    "description": "Respect .gitignore rules (default: true)",
                    "default": true
                }
            },
            "required": ["path"],
            "example": {
                "path": "src/",
                "depth": 2,
                "show_size": true
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = match args["path"].as_str() {
            Some(p) => p,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "tree",
                    "path is required",
                    Some(vec!["path"]),
                    Some(json!({"path": "src/"})),
                ));
            }
        };
        let max_depth = args["depth"].as_u64().unwrap_or(3) as usize;
        let show_hidden = args["show_hidden"].as_bool().unwrap_or(false);
        let show_size = args["show_size"].as_bool().unwrap_or(false);
        let respect_gitignore = args["gitignore"].as_bool().unwrap_or(true);

        let mut output = Vec::new();
        let root_path = Path::new(path);
        
        // Add root directory
        output.push(format!("{}/", root_path.file_name().unwrap_or_default().to_string_lossy()));

        let mut file_count = 0;
        let mut dir_count = 0;

        // Build tree recursively
        build_tree(
            root_path,
            "",
            0,
            max_depth,
            show_hidden,
            show_size,
            respect_gitignore,
            &mut output,
            &mut file_count,
            &mut dir_count,
        ).await?;

        output.push(String::new());
        output.push(format!("{} directories, {} files", dir_count, file_count));

        Ok(ToolResult::success(output.join("\n"))
            .with_metadata("directories", json!(dir_count))
            .with_metadata("files", json!(file_count)))
    }
}

/// Entry with resolved metadata for sorting
struct TreeEntry {
    name: String,
    path: std::path::PathBuf,
    is_dir: bool,
    size: u64,
}

/// Helper function to build tree recursively
async fn build_tree(
    path: &Path,
    prefix: &str,
    depth: usize,
    max_depth: usize,
    show_hidden: bool,
    show_size: bool,
    respect_gitignore: bool,
    output: &mut Vec<String>,
    file_count: &mut usize,
    dir_count: &mut usize,
) -> Result<()> {
    if depth >= max_depth {
        return Ok(());
    }

    // Read directory and collect entries with their metadata
    let mut entries: Vec<TreeEntry> = Vec::new();
    
    let mut dir = match fs::read_dir(path).await {
        Ok(d) => d,
        Err(_) => return Ok(()),
    };

    while let Ok(Some(entry)) = dir.next_entry().await {
        let name = entry.file_name().to_string_lossy().to_string();
        
        // Skip hidden files unless requested
        if !show_hidden && name.starts_with('.') {
            continue;
        }

        // Skip common ignored directories
        if respect_gitignore {
            let skip_dirs = ["node_modules", "target", ".git", "__pycache__", ".venv", "dist", ".next", "vendor"];
            if skip_dirs.contains(&name.as_str()) {
                continue;
            }
        }

        let file_type = match entry.file_type().await {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        
        let size = if show_size {
            entry.metadata().await.map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        entries.push(TreeEntry {
            name,
            path: entry.path(),
            is_dir: file_type.is_dir(),
            size,
        });
    }

    // Sort entries: directories first, then files, alphabetically
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    let total = entries.len();
    for (idx, entry) in entries.iter().enumerate() {
        let is_last = idx == total - 1;
        let connector = if is_last { "└── " } else { "├── " };
        
        let mut line = format!("{}{}", prefix, connector);
        
        if entry.is_dir {
            *dir_count += 1;
            line.push_str(&format!("{}/", entry.name));
        } else {
            *file_count += 1;
            if show_size {
                let size = format_size(entry.size);
                line.push_str(&format!("{} ({})", entry.name, size));
            } else {
                line.push_str(&entry.name);
            }
        }
        
        output.push(line);

        // Recurse into directories
        if entry.is_dir {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            Box::pin(build_tree(
                &entry.path,
                &new_prefix,
                depth + 1,
                max_depth,
                show_hidden,
                show_size,
                respect_gitignore,
                output,
                file_count,
                dir_count,
            )).await?;
        }
    }

    Ok(())
}

/// Format file size in human-readable form
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

/// File information tool - get metadata about a file
pub struct FileInfoTool;

impl FileInfoTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for FileInfoTool {
    fn id(&self) -> &str {
        "fileinfo"
    }

    fn name(&self) -> &str {
        "File Info"
    }

    fn description(&self) -> &str {
        "fileinfo(path: string) - Get detailed information about a file: size, type, permissions, line count, encoding detection, and language."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to inspect"
                }
            },
            "required": ["path"],
            "example": {
                "path": "src/main.rs"
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = match args["path"].as_str() {
            Some(p) => p,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "fileinfo",
                    "path is required",
                    Some(vec!["path"]),
                    Some(json!({"path": "src/main.rs"})),
                ));
            }
        };

        let path_obj = Path::new(path);
        let metadata = fs::metadata(path).await?;

        let mut info = Vec::new();
        
        // Basic info
        info.push(format!("Path: {}", path));
        info.push(format!("Size: {} ({} bytes)", format_size(metadata.len()), metadata.len()));
        
        // File type
        let file_type = if metadata.is_dir() {
            "directory"
        } else if metadata.is_symlink() {
            "symlink"
        } else {
            "file"
        };
        info.push(format!("Type: {}", file_type));

        // Permissions (Unix)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            info.push(format!("Permissions: {:o}", mode & 0o777));
        }

        // Modified time
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                let secs = duration.as_secs();
                info.push(format!("Modified: {} seconds since epoch", secs));
            }
        }

        // For files, get additional info
        if metadata.is_file() {
            // Detect language from extension
            if let Some(ext) = path_obj.extension() {
                let lang = match ext.to_str().unwrap_or("") {
                    "rs" => "Rust",
                    "py" => "Python",
                    "js" => "JavaScript",
                    "ts" => "TypeScript",
                    "tsx" => "TypeScript (React)",
                    "jsx" => "JavaScript (React)",
                    "go" => "Go",
                    "java" => "Java",
                    "c" | "h" => "C",
                    "cpp" | "hpp" | "cc" | "cxx" => "C++",
                    "rb" => "Ruby",
                    "php" => "PHP",
                    "swift" => "Swift",
                    "kt" | "kts" => "Kotlin",
                    "scala" => "Scala",
                    "cs" => "C#",
                    "md" => "Markdown",
                    "json" => "JSON",
                    "yaml" | "yml" => "YAML",
                    "toml" => "TOML",
                    "xml" => "XML",
                    "html" => "HTML",
                    "css" => "CSS",
                    "scss" | "sass" => "SCSS/Sass",
                    "sql" => "SQL",
                    "sh" | "bash" | "zsh" => "Shell",
                    _ => "Unknown",
                };
                info.push(format!("Language: {}", lang));
            }

            // Try to read and count lines
            if let Ok(content) = fs::read_to_string(path).await {
                let lines = content.lines().count();
                let chars = content.chars().count();
                let words = content.split_whitespace().count();
                
                info.push(format!("Lines: {}", lines));
                info.push(format!("Words: {}", words));
                info.push(format!("Characters: {}", chars));

                // Check if it looks like UTF-8 text
                info.push("Encoding: UTF-8 (text)".to_string());
            } else {
                info.push("Encoding: Binary or non-UTF-8".to_string());
            }
        }

        Ok(ToolResult::success(info.join("\n")))
    }
}

/// Head/Tail tool - quickly peek at file beginning or end
pub struct HeadTailTool;

impl HeadTailTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for HeadTailTool {
    fn id(&self) -> &str {
        "headtail"
    }

    fn name(&self) -> &str {
        "Head/Tail"
    }

    fn description(&self) -> &str {
        "headtail(path: string, head?: int, tail?: int) - Quickly peek at the beginning and/or end of a file. Useful for understanding file structure without reading the entire file."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file"
                },
                "head": {
                    "type": "integer",
                    "description": "Number of lines from the beginning (default: 10)",
                    "default": 10
                },
                "tail": {
                    "type": "integer",
                    "description": "Number of lines from the end (default: 0, set to show tail)",
                    "default": 0
                }
            },
            "required": ["path"],
            "example": {
                "path": "src/main.rs",
                "head": 20,
                "tail": 10
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let path = match args["path"].as_str() {
            Some(p) => p,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "headtail",
                    "path is required",
                    Some(vec!["path"]),
                    Some(json!({"path": "src/main.rs", "head": 10})),
                ));
            }
        };
        let head_lines = args["head"].as_u64().unwrap_or(10) as usize;
        let tail_lines = args["tail"].as_u64().unwrap_or(0) as usize;

        let content = fs::read_to_string(path).await?;
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let mut output = Vec::new();
        output.push(format!("=== {} ({} lines total) ===", path, total_lines));
        output.push(String::new());

        // Head
        if head_lines > 0 {
            output.push(format!("--- First {} lines ---", head_lines.min(total_lines)));
            for (i, line) in lines.iter().take(head_lines).enumerate() {
                output.push(format!("{:4} | {}", i + 1, line));
            }
        }

        // Check if there's a gap between head and tail
        let head_end = head_lines;
        let tail_start = total_lines.saturating_sub(tail_lines);
        
        if tail_lines > 0 && tail_start > head_end {
            output.push(String::new());
            output.push(format!("... ({} lines omitted) ...", tail_start - head_end));
            output.push(String::new());
            output.push(format!("--- Last {} lines ---", tail_lines.min(total_lines)));
            for (i, line) in lines.iter().skip(tail_start).enumerate() {
                output.push(format!("{:4} | {}", tail_start + i + 1, line));
            }
        } else if tail_lines > 0 && tail_start <= head_end {
            // Overlap or contiguous - just show everything
            if head_end < total_lines {
                for (i, line) in lines.iter().skip(head_end).enumerate() {
                    output.push(format!("{:4} | {}", head_end + i + 1, line));
                }
            }
        }

        Ok(ToolResult::success(output.join("\n"))
            .with_metadata("total_lines", json!(total_lines)))
    }
}

/// Diff tool - compare files or show git diff
pub struct DiffTool;

impl DiffTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for DiffTool {
    fn id(&self) -> &str {
        "diff"
    }

    fn name(&self) -> &str {
        "Diff"
    }

    fn description(&self) -> &str {
        "diff(file1?: string, file2?: string, git?: bool, staged?: bool) - Compare two files or show git changes. Use git=true for uncommitted changes, staged=true for staged changes."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "file1": {
                    "type": "string",
                    "description": "First file to compare (or file for git diff)"
                },
                "file2": {
                    "type": "string",
                    "description": "Second file to compare"
                },
                "git": {
                    "type": "boolean",
                    "description": "Show git diff for uncommitted changes (default: false)",
                    "default": false
                },
                "staged": {
                    "type": "boolean",
                    "description": "Show git diff for staged changes (default: false)",
                    "default": false
                },
                "context": {
                    "type": "integer",
                    "description": "Lines of context around changes (default: 3)",
                    "default": 3
                }
            },
            "example": {
                "git": true,
                "file1": "src/main.rs"
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let git_mode = args["git"].as_bool().unwrap_or(false);
        let staged = args["staged"].as_bool().unwrap_or(false);
        let context = args["context"].as_u64().unwrap_or(3);

        if git_mode {
            // Git diff mode
            let mut cmd = tokio::process::Command::new("git");
            cmd.arg("diff");
            
            if staged {
                cmd.arg("--staged");
            }
            
            cmd.arg(format!("-U{}", context));

            if let Some(file) = args["file1"].as_str() {
                cmd.arg("--").arg(file);
            }

            let output = cmd.output().await?;
            
            if output.status.success() {
                let diff = String::from_utf8_lossy(&output.stdout);
                if diff.is_empty() {
                    Ok(ToolResult::success("No changes detected"))
                } else {
                    Ok(ToolResult::success(diff.to_string()))
                }
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                Ok(ToolResult::error(format!("Git diff failed: {}", error)))
            }
        } else {
            // File comparison mode
            let file1 = match args["file1"].as_str() {
                Some(f) => f,
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_ARGUMENT",
                        "diff",
                        "file1 is required for file comparison (or use git=true)",
                        Some(vec!["file1"]),
                        Some(json!({"file1": "old.txt", "file2": "new.txt"})),
                    ));
                }
            };
            let file2 = match args["file2"].as_str() {
                Some(f) => f,
                None => {
                    return Ok(ToolResult::structured_error(
                        "INVALID_ARGUMENT",
                        "diff",
                        "file2 is required for file comparison",
                        Some(vec!["file2"]),
                        Some(json!({"file1": file1, "file2": "new.txt"})),
                    ));
                }
            };

            // Use system diff command for better output
            let output = tokio::process::Command::new("diff")
                .arg("-u")
                .arg(format!("--label={}", file1))
                .arg(format!("--label={}", file2))
                .arg(file1)
                .arg(file2)
                .output()
                .await?;

            let diff = String::from_utf8_lossy(&output.stdout);
            if diff.is_empty() && output.status.success() {
                Ok(ToolResult::success("Files are identical"))
            } else {
                Ok(ToolResult::success(diff.to_string()))
            }
        }
    }
}
