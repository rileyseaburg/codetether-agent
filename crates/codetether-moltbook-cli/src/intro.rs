//! Moltbook intro command.
use codetether_moltbook as moltbook;
pub(super) async fn execute() -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    let profile = client.me().await?;
    let (title, content) = moltbook::intro_post(&profile.name);
    let resp = client
        .create_post("introductions", &title, &content)
        .await?;
    println!("✅ CodeTether intro posted to m/introductions!");
    println!("   Title: {}", title);
    tracing::debug!("Post response: {}", resp);
    Ok(())
}
