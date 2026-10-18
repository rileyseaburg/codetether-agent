//! Moltbook update profile command.
use codetether_moltbook as moltbook;
pub(super) async fn execute(
    upd: codetether_cli_options::MoltbookUpdateProfileArgs,
) -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    client.update_profile(upd.description.as_deref()).await?;
    println!("✅ Profile updated (CodeTether branding included).");
    Ok(())
}
