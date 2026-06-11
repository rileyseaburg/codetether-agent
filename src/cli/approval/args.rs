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
    List,
    /// Show one approval request and its decision.
    Show { id: String },
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
