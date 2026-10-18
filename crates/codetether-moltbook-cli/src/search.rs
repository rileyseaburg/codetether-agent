//! Moltbook search command.
use codetether_moltbook as moltbook;
pub(super) async fn execute(s: codetether_cli_options::MoltbookSearchArgs) -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    let results = client.search(&s.query, s.limit).await?;
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
