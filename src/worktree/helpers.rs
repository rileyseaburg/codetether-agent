//! Utility functions for worktree management
use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};

fn sanitize_repo_tag(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_dash = false;
    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    let cleaned = out.trim_matches('-').to_string();
    if cleaned.is_empty() {
        "workspace".to_string()
    } else {
        cleaned
    }
}

pub fn default_base_dir_for_repo(repo_path: &Path) -> PathBuf {
    let repo_name = repo_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(sanitize_repo_tag)
        .unwrap_or_else(|| "workspace".to_string());
    repo_path
        .parent()
        .unwrap_or(repo_path)
        .join(format!(".{repo_name}.codetether-worktrees"))
}

pub fn validate_worktree_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow!("Worktree name cannot be empty"));
    }
    if name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        Ok(())
    } else {
        Err(anyhow!(
            "Invalid worktree name '{}'. Only alphanumeric, '-' and '_' allowed.",
            name
        ))
    }
}

pub(crate) fn summarize_git_output(output: &str) -> String {
    output
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(|l| l.chars().take(220).collect::<String>())
        .unwrap_or_else(|| "no details".to_string())
}

pub(crate) fn combined_output(stdout: &[u8], stderr: &[u8]) -> String {
    format!(
        "{}\n{}",
        String::from_utf8_lossy(stdout),
        String::from_utf8_lossy(stderr)
    )
}
