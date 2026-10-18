//! Moltbook register command.
use codetether_moltbook as moltbook;
pub(super) async fn execute(
    reg: codetether_cli_options::MoltbookRegisterArgs,
) -> anyhow::Result<()> {
    println!("🦞 Registering on Moltbook as '{}'...\n", reg.name);
    let result = moltbook::MoltbookClient::register(&reg.name, reg.description.as_deref()).await?;

    println!("✅ Registered successfully!\n");
    println!("   Agent:             {}", reg.name);
    println!("   Claim URL:         {}", result.agent.claim_url);
    println!("   Verification Code: {}", result.agent.verification_code);
    println!("\n🔗 Send the claim URL to your human to verify ownership.");
    if result.vault_saved {
        println!("🔐 API key has been saved to Vault (codetether/moltbook).");
    } else {
        println!(
            "⚠️  Could not save API key to Vault. Check VAULT_ADDR/VAULT_TOKEN and save it manually."
        );
    }
    Ok(())
}
