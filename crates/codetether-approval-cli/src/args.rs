//! Command schemas for approval inspection and decisions.

use clap::{Parser, Subcommand};

/// Arguments for the approval command.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let args = codetether_approval_cli::ApprovalArgs::parse_from(["approval", "list"]);
/// assert!(matches!(args.command, codetether_approval_cli::ApprovalCommand::List { json: false }));
/// ```
#[derive(Parser, Debug)]
pub struct ApprovalArgs {
    /// Approval store operation to run.
    #[command(subcommand)]
    pub command: ApprovalCommand,
}

/// Approval store subcommands for viewing records and recording decisions.
///
/// # Examples
/// ```
/// let command = codetether_approval_cli::ApprovalCommand::List { json: true };
/// match command {
///     codetether_approval_cli::ApprovalCommand::List { json } => assert!(json),
///     _ => unreachable!(),
/// }
/// ```
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
        /// Approval request identifier.
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

/// Arguments shared by approve and deny operations.
///
/// # Examples
/// ```
/// let args = codetether_approval_cli::ApprovalDecisionArgs {
///     id: "request-id".into(), actor: Some("operator".into()), reason: None,
/// };
/// assert_eq!(args.actor.as_deref(), Some("operator"));
/// ```
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
