//! Moltbook profile command.
use codetether_moltbook as moltbook;
pub(super) async fn execute() -> anyhow::Result<()> {
    let client = moltbook::MoltbookClient::from_vault_or_env().await?;
    let profile = client.me().await?;
    println!("# Moltbook Profile\n");
    println!("**Name:** {}", profile.name);
    if let Some(desc) = &profile.description {
        println!("**Description:** {}", desc);
    }
    if let Some(k) = profile.karma {
        println!("**Karma:** {}", k);
    }
    if let Some(f) = profile.follower_count {
        println!("**Followers:** {}", f);
    }
    if let Some(claimed) = profile.is_claimed {
        println!("**Claimed:** {}", if claimed { "Yes" } else { "No" });
    }
    println!("\n🔗 https://www.moltbook.com/u/{}", profile.name);
    Ok(())
}
