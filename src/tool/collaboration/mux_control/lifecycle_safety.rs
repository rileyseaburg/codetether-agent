//! Safety gate for mux lifecycle mutations.

use anyhow::{Context, Result, bail};

use super::args::Args;

pub(super) async fn ensure(args: &Args) -> Result<()> {
    let name = args.name.as_deref().context("name required")?;
    let status = crate::mux::control::agent_sessions()
        .await?
        .into_iter()
        .find(|item| item["name"] == name)
        .context("mux session not found")?;
    if status["status"] == "working" {
        bail!("refusing lifecycle action on a working mux");
    }
    if status["status"].is_null() && !args.force {
        bail!("mux runtime is unknown; force=true is required");
    }
    Ok(())
}
