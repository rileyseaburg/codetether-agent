//! Moltbook heartbeat command.
use codetether_moltbook as moltbook;
pub(super) async fn execute() -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    let summary = client.heartbeat().await?;
    println!("{}", summary);
    Ok(())
}
