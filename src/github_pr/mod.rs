pub mod autopilot;
mod body;
pub mod ci_watcher;
mod client;
mod context;
pub mod provenance_block;

use crate::cli::{PrArgs, PrCommand};

pub async fn run(args: PrArgs) -> anyhow::Result<()> {
    match args.command {
        PrCommand::Create(args) => {
            let result = client::create_or_update(args).await?;
            if result.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("PR_URL={}", result.html_url);
                println!("PR_NUMBER={}", result.number);
                println!("PR_STATE={}", result.state);
                println!("PR_DRAFT={}", result.draft);
                println!("PR_HEAD={}", result.head_sha);
            }
            Ok(())
        }
    }
}
