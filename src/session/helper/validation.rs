use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use tokio::process::Command;

const MAX_DIAGNOSTICS_PER_FILE: usize = 25;

#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub issue_count: usize,
    pub prompt: String,
}

pub async fn capture_git_dirty_files(workspace_dir: &Path) -> HashSet<PathBuf> {
    let mut files = HashSet::new();
    for args in [
        &["diff", "--name-only", "--relative", "--"][..],
        &["diff", "--cached", "--name-only", "--relative", "--"][..],
        &["ls-files", "--others", "--exclude-standard"][..],
    ] {
        let output = Command::new("git")
            .args(args)
            .current_dir(workspace_dir)
            .output()
            .await;
        let Ok(output) = output else {
            continue;
        };
        if !output.status.success() {
            continue;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            files.insert(normalize_workspace_path(workspace_dir, line));
        }
    }
    files
}

pub fn track_touched_files(
    touched_files: &mut HashSet<PathBuf>,
    workspace_dir: &Path,
    tool_name: &str,
    tool_input: &Value,
    tool_metadata: Option<&HashMap<String, Value>>,
) {
    if !is_mutating_tool(tool_name) {
        return;
    }

    let mut files = Vec::new();

    match tool_name {
        "write" | "edit" | "confirm_edit" => {
            if let Some(path) = string_field(tool_input, &["path"]) {
                files.push(path.to_string());
            }
        }
        "advanced_edit" => {
            if let Some(path) = string_field(tool_input, &["filePath", "file_path", "path"]) {
                files.push(path.to_string());
            }
        }
        "multiedit" | "confirm_multiedit" => {
            if let Some(edits) = tool_input.get("edits").and_then(Value::as_array) {
                for edit in edits {
                    if let Some(path) = string_field(edit, &["path", "filePath", "file_path"]) {
                        files.push(path.to_string());
                    }
                }
            }
        }
        "patch" => {
            collect_metadata_paths(&mut files, tool_metadata, &["files"]);
        }
        _ => {}
    }

    collect_metadata_paths(&mut files, tool_metadata, &["path", "file"]);

    for file in files {
        touched_files.insert(normalize_workspace_path(workspace_dir, &file));
    }
}

pub async fn build_validation_report(
    workspace_dir: &Path,
    touched_files: &HashSet<PathBuf>,
    baseline_git_dirty_files: &HashSet<PathBuf>,
) -> Result<Option<ValidationReport>> {
    let mut candidate_files = touched_files.clone();
    let current_git_dirty = capture_git_dirty_files(workspace_dir).await;
    candidate_files.extend(
        current_git_dirty
            .difference(baseline_git_dirty_files)
            .cloned(),
    );

    let mut existing_files: Vec<PathBuf> = candidate_files
        .into_iter()
        .filter(|path| path.is_file())
        .collect();
    existing_files.sort();
    existing_files.dedup();

    if existing_files.is_empty() {
        return Ok(None);
    }

    let mut issues_by_file = BTreeMap::new();
    let mut issue_count = 0usize;

    for path in existing_files {
        let mut rendered = collect_external_linter_diagnostics(workspace_dir, &path).await;

        rendered.sort();
        rendered.dedup();

        if !rendered.is_empty() {
            issue_count += rendered.len();
            issues_by_file.insert(relative_display_path(workspace_dir, &path), rendered);
        }
    }

    if issues_by_file.is_empty() {
        return Ok(None);
    }

    let mut prompt = String::from(
        "Mandatory post-edit verification found unresolved diagnostics in files you changed. \
Do not finish yet. Fix every issue below, respecting workspace config files such as eslint, biome, \
ruff, stylelint, tsconfig, and other project-local tooling. After fixing them, re-check the same \
files and only then provide the final answer.\n\n",
    );

    for (path, diagnostics) in issues_by_file {
        prompt.push_str(&format!("{path}\n"));
        for diagnostic in diagnostics.iter().take(MAX_DIAGNOSTICS_PER_FILE) {
            prompt.push_str("  - ");
            prompt.push_str(diagnostic);
            prompt.push('\n');
        }
        if diagnostics.len() > MAX_DIAGNOSTICS_PER_FILE {
            prompt.push_str(&format!(
                "  - ... {} more diagnostics omitted\n",
                diagnostics.len() - MAX_DIAGNOSTICS_PER_FILE
            ));
        }
        prompt.push('\n');
    }

    Ok(Some(ValidationReport {
        issue_count,
        prompt: prompt.trim_end().to_string(),
    }))
}
async fn collect_external_linter_diagnostics(workspace_dir: &Path, path: &Path) -> Vec<String> {
    let ext = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();
    if !matches!(ext, "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs") {
        return Vec::new();
    }

    let relative_path = path
        .strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string();
    let output = Command::new("npx")
        .args(["--no-install", "eslint", "--format", "json", &relative_path])
        .current_dir(workspace_dir)
        .output()
        .await;
    let Ok(output) = output else {
        return Vec::new();
    };
    if output.stdout.is_empty() {
        return Vec::new();
    }

    let reports: Result<Vec<EslintFileReport>, _> = serde_json::from_slice(&output.stdout);
    let Ok(reports) = reports else {
        return Vec::new();
    };

    reports
        .into_iter()
        .flat_map(|report| {
            let file_path = report.file_path;
            report.messages.into_iter().map(move |message| {
                let severity = match message.severity {
                    2 => "error",
                    1 => "warning",
                    _ => "info",
                };
                let code = message
                    .rule_id
                    .as_deref()
                    .map(|rule_id| format!(" ({rule_id})"))
                    .unwrap_or_default();
                format!(
                    "[{severity}] {}:{}:{} [eslint-cli]{} {}",
                    relative_display_path(workspace_dir, Path::new(&file_path)),
                    message.line,
                    message.column,
                    code,
                    message.message.replace('\n', " ")
                )
            })
        })
        .collect()
}

fn is_mutating_tool(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "write"
            | "edit"
            | "advanced_edit"
            | "confirm_edit"
            | "multiedit"
            | "confirm_multiedit"
            | "patch"
    )
}

fn collect_metadata_paths(
    files: &mut Vec<String>,
    tool_metadata: Option<&HashMap<String, Value>>,
    keys: &[&str],
) {
    let Some(tool_metadata) = tool_metadata else {
        return;
    };

    for key in keys {
        let Some(value) = tool_metadata.get(*key) else {
            continue;
        };

        match value {
            Value::String(path) => files.push(path.clone()),
            Value::Array(paths) => {
                for path in paths.iter().filter_map(Value::as_str) {
                    files.push(path.to_string());
                }
            }
            _ => {}
        }
    }
}

fn string_field<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_str))
}

fn normalize_workspace_path(workspace_dir: &Path, raw_path: &str) -> PathBuf {
    let path = PathBuf::from(raw_path);
    if path.is_absolute() {
        path
    } else {
        workspace_dir.join(path)
    }
}

fn relative_display_path(workspace_dir: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[derive(Debug, Deserialize)]
struct EslintFileReport {
    #[serde(rename = "filePath")]
    file_path: String,
    messages: Vec<EslintMessage>,
}

#[derive(Debug, Deserialize)]
struct EslintMessage {
    #[serde(rename = "ruleId")]
    rule_id: Option<String>,
    severity: u8,
    message: String,
    line: u32,
    column: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn tracks_edit_paths_from_arguments() {
        let workspace_dir = Path::new("/workspace");
        let mut touched_files = HashSet::new();
        track_touched_files(
            &mut touched_files,
            workspace_dir,
            "edit",
            &json!({ "path": "src/main.ts" }),
            None,
        );

        assert!(touched_files.contains(&PathBuf::from("/workspace/src/main.ts")));
    }

    #[test]
    fn tracks_patch_paths_from_metadata() {
        let workspace_dir = Path::new("/workspace");
        let mut touched_files = HashSet::new();
        let metadata =
            HashMap::from([("files".to_string(), json!(["src/lib.rs", "tests/app.rs"]))]);

        track_touched_files(
            &mut touched_files,
            workspace_dir,
            "patch",
            &json!({}),
            Some(&metadata),
        );

        assert!(touched_files.contains(&PathBuf::from("/workspace/src/lib.rs")));
        assert!(touched_files.contains(&PathBuf::from("/workspace/tests/app.rs")));
    }
}
