use clap::{Parser, Subcommand};

/// Arguments for `codetether approval`.
#[derive(Parser, Debug)]
pub struct ApprovalArgs {
    /// Approval store operation to run.
    #[command(subcommand)]
    pub command: ApprovalCommand,
}

/// Approval store subcommands.
#[derive(Subcommand, Debug)]
pub enum ApprovalCommand {
    /// List approval requests.
    List {
        /// Emit JSON instead of the aligned text table.
        ///
        /// The text table pads and truncates fields, so a resource path holding a
        /// space cannot be recovered by splitting columns. Scripts and agents
        /// should use this flag rather than parsing the table.
        #[arg(long)]
        json: bool,
    },
    /// Show one approval request and its decision.
    Show {
        id: String,
        /// Emit JSON instead of `key: value` lines.
        #[arg(long)]
        json: bool,
    },
    /// Approve a pending request.
    Approve(ApprovalDecisionArgs),
    /// Deny a pending request.
    Deny(ApprovalDecisionArgs),
}

/// Arguments shared by approval decisions.
#[derive(Parser, Debug)]
pub struct ApprovalDecisionArgs {
    /// Approval request id.
    pub id: String,
    /// Actor recorded on the decision.
    #[arg(long)]
    pub actor: Option<String>,
    /// Human-readable decision reason.
    #[arg(long)]
    pub reason: Option<String>,
}
