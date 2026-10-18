//! Route classified simple commands to their native adapters.
use super::*;
mod inspection;
mod memory;
mod panels;
mod sessions;
mod views;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: SimpleCommand) {
    match kind {
        SimpleCommand::Sessions
        | SimpleCommand::ImportCodex
        | SimpleCommand::New
        | SimpleCommand::Model => sessions::run(host, kind).await,
        SimpleCommand::Bus
        | SimpleCommand::Lsp
        | SimpleCommand::Rlm
        | SimpleCommand::Latency
        | SimpleCommand::Transport
        | SimpleCommand::Inspector
        | SimpleCommand::AuditLoop
        | SimpleCommand::Settings
        | SimpleCommand::Subagents => panels::run(host, kind),
        SimpleCommand::Help
        | SimpleCommand::Swarm
        | SimpleCommand::Ralph
        | SimpleCommand::Chat
        | SimpleCommand::Webview
        | SimpleCommand::Classic
        | SimpleCommand::Symbols => views::run(host, kind),
        SimpleCommand::Audit | SimpleCommand::Git | SimpleCommand::Plugin | SimpleCommand::Keys => {
            inspection::run(host, kind).await
        }
        SimpleCommand::Recall => memory::run(host, kind),
    }
}
