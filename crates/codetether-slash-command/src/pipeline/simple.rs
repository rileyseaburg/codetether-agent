//! Exact simple-command aliases, preserving the existing spellings.
use super::SimpleCommand;
pub(super) fn classify(command: &str) -> Option<SimpleCommand> {
    Some(match command {
        "/help" => SimpleCommand::Help,
        "/sessions" | "/session" => SimpleCommand::Sessions,
        "/import-codex" => SimpleCommand::ImportCodex,
        "/swarm" => SimpleCommand::Swarm,
        "/ralph" => SimpleCommand::Ralph,
        "/bus" | "/protocol" => SimpleCommand::Bus,
        "/model" => SimpleCommand::Model,
        "/lsp" => SimpleCommand::Lsp,
        "/rlm" => SimpleCommand::Rlm,
        "/latency" => SimpleCommand::Latency,
        "/transport" => SimpleCommand::Transport,
        "/inspector" => SimpleCommand::Inspector,
        "/audit" => SimpleCommand::Audit,
        "/git" => SimpleCommand::Git,
        "/auditloop" => SimpleCommand::AuditLoop,
        "/chat" | "/home" | "/main" => SimpleCommand::Chat,
        "/webview" => SimpleCommand::Webview,
        "/classic" => SimpleCommand::Classic,
        "/symbols" | "/symbol" => SimpleCommand::Symbols,
        "/new" => SimpleCommand::New,
        "/recall" => SimpleCommand::Recall,
        "/plugin" => SimpleCommand::Plugin,
        "/keys" => SimpleCommand::Keys,
        "/settings" => SimpleCommand::Settings,
        "/agents" | "/subagents" => SimpleCommand::Subagents,
        _ => return None,
    })
}
