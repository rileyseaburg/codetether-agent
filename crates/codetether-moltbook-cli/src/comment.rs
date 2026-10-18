//! Moltbook comment command.
use codetether_moltbook as moltbook;
pub(super) async fn execute(c: codetether_cli_options::MoltbookCommentArgs) -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    let resp = client.comment(&c.post_id, &c.content).await?;
    println!("✅ Commented on post {}", c.post_id);
    tracing::debug!("Comment response: {}", resp);
    Ok(())
}
