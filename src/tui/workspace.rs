//! Workspace snapshot and git detection helpers

use super::*;

impl WorkspaceSnapshot {
    fn capture(root: &Path, max_entries: usize) -> Self {
        let mut entries: Vec<WorkspaceEntry> = Vec::new();

        if let Ok(read_dir) = std::fs::read_dir(root) {
            for entry in read_dir.flatten() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if should_skip_workspace_entry(&file_name) {
                    continue;
                }

                let kind = match entry.file_type() {
                    Ok(ft) if ft.is_dir() => WorkspaceEntryKind::Directory,
                    _ => WorkspaceEntryKind::File,
                };

                entries.push(WorkspaceEntry {
                    name: file_name,
                    kind,
                });
            }
        }

        entries.sort_by(|a, b| match (a.kind, b.kind) {
            (WorkspaceEntryKind::Directory, WorkspaceEntryKind::File) => std::cmp::Ordering::Less,
            (WorkspaceEntryKind::File, WorkspaceEntryKind::Directory) => {
                std::cmp::Ordering::Greater
            }
            _ => a
                .name
                .to_ascii_lowercase()
                .cmp(&b.name.to_ascii_lowercase()),
        });
        entries.truncate(max_entries);

        Self {
            root_display: root.to_string_lossy().to_string(),
            git_branch: detect_git_branch(root),
            git_dirty_files: detect_git_dirty_files(root),
            entries,
            captured_at: chrono::Local::now().format("%H:%M:%S").to_string(),
        }
    }
}

fn should_skip_workspace_entry(name: &str) -> bool {
    matches!(
        name,
        ".git" | "node_modules" | "target" | ".next" | "__pycache__" | ".venv"
    )
}

fn detect_git_branch(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        None
    } else {
        Some(branch)
    }
}

fn detect_git_dirty_files(root: &Path) -> usize {
    let output = match Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain"])
        .output()
    {
        Ok(out) => out,
        Err(_) => return 0,
    };

    if !output.status.success() {
        return 0;
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn resolve_provider_for_model_autochat(
