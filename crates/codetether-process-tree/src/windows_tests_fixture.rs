//! Windows parent/descendant fixture for cancellation tests.
use std::path::Path;
use tempfile::TempDir;
use tokio::process::Command;
pub(super) fn descendant_command(temp: &TempDir, started: &Path, survived: &Path) -> Command {
    let child_script = temp.path().join("child.ps1");
    let parent_script = temp.path().join("parent.ps1");
    std::fs::write(
        &child_script,
        concat!(
            "Set-Content -LiteralPath $env:CT_TREE_STARTED -Value started\n",
            "Start-Sleep -Milliseconds 1200\n",
            "Set-Content -LiteralPath $env:CT_TREE_SURVIVED -Value survived\n",
        ),
    )
    .expect("write child process script");
    std::fs::write(
        &parent_script,
        concat!(
            "Start-Sleep -Milliseconds 300\n",
            "& powershell.exe -NoProfile -File $env:CT_TREE_CHILD\n",
        ),
    )
    .expect("write parent process script");
    let mut command = Command::new("powershell.exe");
    command
        .args(["-NoProfile", "-File"])
        .arg(parent_script)
        .env("CT_TREE_CHILD", child_script)
        .env("CT_TREE_STARTED", started)
        .env("CT_TREE_SURVIVED", survived);
    command
}
