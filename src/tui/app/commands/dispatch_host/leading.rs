//! Route classified leading commands to their native adapters.
use super::*;
mod attachments;
mod diagnostics;
mod execution;
mod settings;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: LeadingCommand, rest: &str) {
    match kind {
        LeadingCommand::Image | LeadingCommand::File | LeadingCommand::Edit => {
            attachments::run(host, kind, rest)
        }
        LeadingCommand::Go | LeadingCommand::Ask => execution::run(host, kind, rest).await,
        LeadingCommand::AutoApply | LeadingCommand::Network | LeadingCommand::Autocomplete => {
            settings::run(host, kind, rest).await
        }
        LeadingCommand::Context | LeadingCommand::Mcp => diagnostics::run(host, kind, rest).await,
    }
}
