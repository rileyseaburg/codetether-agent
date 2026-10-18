//! Moltbook status command.
use codetether_moltbook as moltbook;
pub(super) async fn execute() -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    let status = client.claim_status().await?;
    println!("Moltbook claim status: {}", status.status);
    Ok(())
}
