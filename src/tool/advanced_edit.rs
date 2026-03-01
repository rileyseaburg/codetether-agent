//! Advanced Edit Tool with multiple replacement strategies (advanced edit tool)

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::path::PathBuf;
use tokio::fs;

pub struct AdvancedEditTool;

impl Default for AdvancedEditTool {
    fn default() -> Self {
        Self::new()
    }
}

impl AdvancedEditTool {
    pub fn new() -> Self {
        Self
    }
}

/// Levenshtein distance for fuzzy matching
fn levenshtein(a: &str, b: &str) -> usize {
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut matrix = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        matrix[i][0] = i;
    }
    for j in 0..=b.len() {
        matrix[0][j] = j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    matrix[a.len()][b.len()]
}

type Replacer = fn(&str, &str) -> Vec<String>;

/// Simple exact match
fn simple_replacer(content: &str, find: &str) -> Vec<String> {
    if content.contains(find) {
        vec![find.to_string()]
    } else {
        vec![]
    }
}

/// Match with trimmed lines
fn line_trimmed_replacer(content: &str, find: &str) -> Vec<String> {
    let orig_lines: Vec<&str> = content.lines().collect();
    let mut search_lines: Vec<&str> = find.lines().collect();
    if search_lines.last() == Some(&"") {
        search_lines.pop();
    }
    let mut results = vec![];
    for i in 0..=orig_lines.len().saturating_sub(search_lines.len()) {
        let mut matches = true;
        for j in 0..search_lines.len() {
            if orig_lines.get(i + j).map(|l| l.trim()) != Some(search_lines[j].trim()) {
                matches = false;
                break;
            }
        }
        if matches {
            let matched: Vec<&str> = orig_lines[i..i + search_lines.len()].to_vec();
            results.push(matched.join("\n"));
        }
    }
    results
}

/// Block anchor matching (first/last line anchors)
fn block_anchor_replacer(content: &str, find: &str) -> Vec<String> {
    let orig_lines: Vec<&str> = content.lines().collect();
    let mut search_lines: Vec<&str> = find.lines().collect();
    if search_lines.len() < 3 {
        return vec![];
    }
    if search_lines.last() == Some(&"") {
        search_lines.pop();
    }
    let first = search_lines[0].trim();
    let last = search_lines.last().unwrap().trim();
    let mut candidates = vec![];
    for i in 0..orig_lines.len() {
        if orig_lines[i].trim() != first {
            continue;
        }
        for j in (i + 2)..orig_lines.len() {
            if orig_lines[j].trim() == last {
                candidates.push((i, j));
                break;
            }
        }
    }
    if candidates.is_empty() {
        return vec![];
    }
    if candidates.len() == 1 {
        let (start, end) = candidates[0];
        return vec![orig_lines[start..=end].join("\n")];
    }
    // Multiple candidates: find best by similarity
    let mut best = None;
    let mut best_sim = -1.0f64;
    for (start, end) in candidates {
        let block_size = end - start + 1;
        let mut sim = 0.0;
        let lines_to_check = (search_lines.len() - 2).min(block_size - 2);
        if lines_to_check > 0 {
            for j in 1..search_lines.len().min(block_size) - 1 {
                let orig = orig_lines[start + j].trim();
                let search = search_lines[j].trim();
                let max_len = orig.len().max(search.len());
                if max_len > 0 {
                    let dist = levenshtein(orig, search);
                    sim += 1.0 - (dist as f64 / max_len as f64);
                }
            }
            sim /= lines_to_check as f64;
        } else {
            sim = 1.0;
        }
        if sim > best_sim {
            best_sim = sim;
            best = Some((start, end));
        }
    }
    if best_sim >= 0.3
        && let Some((s, e)) = best
    {
        return vec![orig_lines[s..=e].join("\n")];
    }
    vec![]
}

/// Whitespace normalized matching
fn whitespace_normalized_replacer(content: &str, find: &str) -> Vec<String> {
    let normalize = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    let norm_find = normalize(find);
    let mut results = vec![];
    for line in content.lines() {
        if normalize(line) == norm_find {
            results.push(line.to_string());
        }
    }
    // Multi-line
    let find_lines: Vec<&str> = find.lines().collect();
    if find_lines.len() > 1 {
        let lines: Vec<&str> = content.lines().collect();
        for i in 0..=lines.len().saturating_sub(find_lines.len()) {
            let block = lines[i..i + find_lines.len()].join("\n");
            if normalize(&block) == norm_find {
                results.push(block);
            }
        }
    }
    results
}

/// Indentation flexible matching
fn indentation_flexible_replacer(content: &str, find: &str) -> Vec<String> {
    let remove_indent = |s: &str| {
        let lines: Vec<&str> = s.lines().collect();
        let non_empty: Vec<_> = lines.iter().filter(|l| !l.trim().is_empty()).collect();
        if non_empty.is_empty() {
            return s.to_string();
        }
        let min_indent = non_empty
            .iter()
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);
        lines
            .iter()
            .map(|l| {
                if l.len() >= min_indent {
                    &l[min_indent..]
                } else {
                    *l
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let norm_find = remove_indent(find);
    let lines: Vec<&str> = content.lines().collect();
    let find_lines: Vec<&str> = find.lines().collect();
    let mut results = vec![];
    for i in 0..=lines.len().saturating_sub(find_lines.len()) {
        let block = lines[i..i + find_lines.len()].join("\n");
        if remove_indent(&block) == norm_find {
            results.push(block);
        }
    }
    results
}

/// Trimmed boundary matching
fn trimmed_boundary_replacer(content: &str, find: &str) -> Vec<String> {
    let trimmed = find.trim();
    if trimmed == find {
        return vec![];
    }
    if content.contains(trimmed) {
        return vec![trimmed.to_string()];
    }
    vec![]
}

/// Apply replacement with multiple strategies
fn replace(content: &str, old: &str, new: &str, replace_all: bool) -> Result<String> {
    if old == new {
        anyhow::bail!("oldString and newString must be different");
    }
    let replacers: Vec<Replacer> = vec![
        simple_replacer,
        line_trimmed_replacer,
        block_anchor_replacer,
        whitespace_normalized_replacer,
        indentation_flexible_replacer,
        trimmed_boundary_replacer,
    ];
    for replacer in replacers {
        let matches = replacer(content, old);
        for search in matches {
            if !content.contains(&search) {
                continue;
            }
            if replace_all {
                return Ok(content.replace(&search, new));
            }
            let first = content.find(&search);
            let last = content.rfind(&search);
            if first != last {
                continue; // Multiple matches, need more context
            }
            if let Some(idx) = first {
                return Ok(format!(
                    "{}{}{}",
                    &content[..idx],
                    new,
                    &content[idx + search.len()..]
                ));
            }
        }
    }
    anyhow::bail!("oldString not found in content. Provide more context or check for typos.")
}

#[async_trait]
impl Tool for AdvancedEditTool {
    fn id(&self) -> &str {
        "edit"
    }
    fn name(&self) -> &str {
        "Edit"
    }
    fn description(&self) -> &str {
        "Edit a file by replacing oldString with newString. Uses multiple matching strategies \
         including exact match, line-trimmed, block anchor, whitespace normalized, and \
         indentation flexible matching. Fails if match is ambiguous."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "filePath": {"type": "string", "description": "Absolute path to file"},
                "oldString": {"type": "string", "description": "Text to replace"},
                "newString": {"type": "string", "description": "Replacement text"},
                "replaceAll": {"type": "boolean", "description": "Replace all occurrences", "default": false}
            },
            "required": ["filePath", "oldString", "newString"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let example = json!({
            "filePath": "/absolute/path/to/file.rs",
            "oldString": "text to find",
            "newString": "replacement text"
        });

        let file_path = match params.get("filePath").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "edit",
                    "filePath is required and must be a non-empty string (absolute path to the file)",
                    Some(vec!["filePath"]),
                    Some(example),
                ));
            }
        };
        let old_string = match params.get("oldString").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "edit",
                    "oldString is required (the exact text to find and replace)",
                    Some(vec!["oldString"]),
                    Some(json!({
                        "filePath": file_path,
                        "oldString": "text to find in file",
                        "newString": "replacement text"
                    })),
                ));
            }
        };
        let new_string = match params.get("newString").and_then(|v| v.as_str()) {
            Some(s) => s.to_string(),
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "edit",
                    "newString is required (the text to replace oldString with)",
                    Some(vec!["newString"]),
                    Some(json!({
                        "filePath": file_path,
                        "oldString": old_string,
                        "newString": "replacement text"
                    })),
                ));
            }
        };
        let replace_all = params
            .get("replaceAll")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let path = PathBuf::from(&file_path);
        if !path.exists() {
            return Ok(ToolResult::structured_error(
                "FILE_NOT_FOUND",
                "edit",
                &format!("File not found: {file_path}"),
                None,
                Some(json!({
                    "hint": "Use an absolute path. List directory contents first to verify the file exists.",
                    "filePath": file_path
                })),
            ));
        }
        if old_string == new_string {
            return Ok(ToolResult::error(
                "oldString and newString must be different",
            ));
        }
        // Creating new file
        if old_string.is_empty() {
            fs::write(&path, &new_string).await?;
            return Ok(ToolResult::success(format!("Created file: {file_path}")));
        }
        let content = fs::read_to_string(&path).await?;
        let new_content = match replace(&content, &old_string, &new_string, replace_all) {
            Ok(c) => c,
            Err(_) => {
                return Ok(ToolResult::structured_error(
                    "NOT_FOUND",
                    "edit",
                    "oldString not found in file content. Provide more surrounding context or check for typos, whitespace, and indentation.",
                    None,
                    Some(json!({
                        "hint": "Read the file first to see its exact content, then copy the text you want to replace verbatim including whitespace.",
                        "filePath": file_path,
                        "oldString": "<copy exact text from file including whitespace and indentation>",
                        "newString": "replacement text"
                    })),
                ));
            }
        };
        fs::write(&path, &new_content).await?;
        let old_lines = old_string.lines().count();
        let new_lines = new_string.lines().count();
        Ok(ToolResult::success(format!(
            "Edit applied: {old_lines} line(s) replaced with {new_lines} line(s) in {file_path}"
        ))
        .with_metadata("file", json!(file_path)))
    }
}
