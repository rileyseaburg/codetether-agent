//! Monarch authentication through the configured credential backend.
mod args;
#[cfg(test)]
mod tests;
pub use args::{MonarchAuthArgs, MonarchCommand};

pub(crate) async fn execute(args: MonarchAuthArgs) -> anyhow::Result<()> {
    match args.command.unwrap_or(MonarchCommand::Login) {
        MonarchCommand::Login => {
            codetether_monarch_auth::login(false).await?;
            println!("Monarch read-only authorization saved to the configured secret backend.");
        }
        MonarchCommand::Status => {
            println!(
                "{}",
                serde_json::json!({
                    "credentials_present": codetether_monarch_auth::status().await?,
                    "backend": "configured_vault", "mode": "read_only"
                })
            );
        }
        MonarchCommand::Logout => {
            codetether_monarch_auth::logout().await?;
            println!("Monarch authorization removed from the configured secret backend.");
        }
    }
    Ok(())
}
