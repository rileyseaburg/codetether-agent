//! Typed simple and trailing command operations.
/// Simple command whose aliases are classified by the pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleCommand {
    /// Execute the help command.
    Help,
    /// Execute the sessions command.
    Sessions,
    /// Execute the importcodex command.
    ImportCodex,
    /// Execute the swarm command.
    Swarm,
    /// Execute the ralph command.
    Ralph,
    /// Execute the bus command.
    Bus,
    /// Execute the model command.
    Model,
    /// Execute the lsp command.
    Lsp,
    /// Execute the rlm command.
    Rlm,
    /// Execute the latency command.
    Latency,
    /// Execute the transport command.
    Transport,
    /// Execute the inspector command.
    Inspector,
    /// Execute the audit command.
    Audit,
    /// Execute the git command.
    Git,
    /// Execute the auditloop command.
    AuditLoop,
    /// Execute the chat command.
    Chat,
    /// Execute the webview command.
    Webview,
    /// Execute the classic command.
    Classic,
    /// Execute the symbols command.
    Symbols,
    /// Execute the new command.
    New,
    /// Execute the recall command.
    Recall,
    /// Execute the plugin command.
    Plugin,
    /// Execute the keys command.
    Keys,
    /// Execute the settings command.
    Settings,
    /// Execute the subagents command.
    Subagents,
}
/// Trailing command with its already-separated argument text.
#[derive(Debug, Clone, Copy)]
pub enum TailCommand {
    /// Execute spawnform.
    SpawnForm,
    /// Execute spawntask.
    SpawnTask,
    /// Execute kill.
    Kill,
    /// Execute agents.
    Agents,
    /// Execute agent.
    Agent,
    /// Execute autochat.
    Autochat,
}
