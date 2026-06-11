use std::{path::Path, process::Command};

use crate::tui::app::state::App;
#[path = "codex_parity_diff_render.rs"]
mod render;

pub(super) fn show(app: &mut App, cwd: &Path) {
    let status = match git(cwd, &["status", "--short"]) {
        Ok(out) => out,
        Err(error) => {
            render::finish(app, format!("Diff unavailable: {error}"));
            return;
        }
    };
    let unstaged = git(cwd, &["diff", "--stat"]).unwrap_or_default();
    let staged = git(cwd, &["diff", "--cached", "--stat"]).unwrap_or_default();
    let text = if status.trim().is_empty() {
        "Working tree clean.".to_string()
    } else {
        format!(
            "Git status\n{}\n\nUnstaged diff\n{}\n\nStaged diff\n{}",
            render::fallback(&status),
            render::fallback(&unstaged),
            render::fallback(&staged),
        )
    };
    render::finish(app, text);
}

fn git(cwd: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}
