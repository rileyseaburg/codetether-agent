//! Moltbook CLI command execution and presentation.
//!
//! execute dispatches typed options to existing client operations. Commands
//! retain their current credential lookup, network calls and output messages.
use codetether_cli_options::{MoltbookArgs, MoltbookCommand};
mod comment;
mod heartbeat;
mod intro;
mod post;
mod profile;
mod register;
mod search;
mod status;
mod update_profile;
/// Execute one Moltbook operation with the existing client behavior.
/// # Arguments
/// * `args` - Parsed operation and its options.
/// # Returns
/// Unit after the operation and output complete.
/// # Errors
/// Propagates client, credential and output serialization failures.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_cli_options::MoltbookArgs) {
/// let _ = codetether_moltbook_cli::execute(args).await;
/// # }
/// ```
pub async fn execute(args: MoltbookArgs) -> anyhow::Result<()> {
    match args.command {
        MoltbookCommand::Register(reg) => register::execute(reg).await,
        MoltbookCommand::Status => status::execute().await,
        MoltbookCommand::Profile => profile::execute().await,
        MoltbookCommand::UpdateProfile(upd) => update_profile::execute(upd).await,
        MoltbookCommand::Post(post) => post::execute(post).await,
        MoltbookCommand::Intro => intro::execute().await,
        MoltbookCommand::Heartbeat => heartbeat::execute().await,
        MoltbookCommand::Comment(c) => comment::execute(c).await,
        MoltbookCommand::Search(s) => search::execute(s).await,
    }
}
