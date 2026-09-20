//! Unix parent/descendant fixture for cancellation tests.
use std::path::Path;
use tempfile::TempDir;
use tokio::process::Command;
pub(super) fn descendant_command(temp: &TempDir, started: &Path, survived: &Path) -> Command {
    let child_script = temp.path().join("child.sh");
    let parent_script = temp.path().join("parent.sh");
    std::fs::write(
        &child_script,
        concat!(
            "#!/bin/sh\n",
            "touch \"$CT_TREE_STARTED\"\n",
            "sleep 1.2\n",
            "touch \"$CT_TREE_SURVIVED\"\n",
        ),
    )
    .expect("write child process script");
    std::fs::write(
        &parent_script,
        concat!("#!/bin/sh\n", "sleep 0.3\n", "sh \"$CT_TREE_CHILD\"\n"),
    )
    .expect("write parent process script");
    let mut command = Command::new("sh");
    command
        .arg(parent_script)
        .env("CT_TREE_CHILD", child_script)
        .env("CT_TREE_STARTED", started)
        .env("CT_TREE_SURVIVED", survived);
    command
}
