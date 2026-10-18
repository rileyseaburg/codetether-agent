//! Moltbook post command.
use codetether_moltbook as moltbook;
pub(super) async fn execute(post: codetether_cli_options::MoltbookPostArgs) -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    let resp = client
        .create_post(&post.submolt, &post.title, &post.content)
        .await?;
    println!("✅ Posted to m/{}: {}", post.submolt, post.title);
    tracing::debug!("Post response: {}", resp);
    Ok(())
}
