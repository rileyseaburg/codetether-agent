//! Fixed executable and explicit child-only project/environment selection.

use std::path::Path;
use tokio::process::Command;

pub(super) fn command(cwd: &Path, environment: &str) -> Command {
    let mut command = Command::new("rustyroad-mcp");
    command
        .current_dir(cwd)
        .env("RUSTYROAD_PROJECT_DIR", cwd)
        .env("ENVIRONMENT", environment)
        .env_remove("ENV");
    command
}
