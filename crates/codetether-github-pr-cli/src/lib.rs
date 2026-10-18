//! CLI-owned GitHub pull-request creation and presentation.
//!
//! [run] preserves native CLI argument, credential and output behavior while
//! using the shared repository-context and GitHub HTTP service.
mod client;
use codetether_cli_options::{PrArgs, PrCommand};

/// Execute the requested pull-request CLI operation.
/// # Arguments
/// * `args` - Parsed PR command and create options.
/// # Returns
/// Unit after writing the original JSON or key/value output.
/// # Errors
/// Returns context, body-file, credential, HTTP or serialization failures.
/// # Examples
/// ```rust,no_run
/// # async fn example(args: codetether_cli_options::PrArgs) -> anyhow::Result<()> {
/// codetether_github_pr_cli::run(args).await
/// # }
/// ```
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
