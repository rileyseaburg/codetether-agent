//! Sandboxed child construction for language-server transports.

use anyhow::Result;
use std::path::Path;

#[path = "transport_spawn_policy.rs"]
mod policy;

pub(super) async fn child(
    command: &str,
    args: &[String],
    workspace: &Path,
) -> Result<tokio::process::Child> {
    let (executable, policy) = policy::resolve(command)?;
    crate::tool::sandbox::sandbox_spawn_piped::spawn(&executable, args, &policy, workspace).await
}
