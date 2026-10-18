//! Operator-facing approval store commands.
//!
//! [ApprovalArgs] selects list, show, approve or deny. [execute] opens the
//! shared approval store and dispatches the request with stable text or JSON
//! rendering; ledger persistence is owned by the approval core crate.

mod args;
mod decisions;
mod defaults;
mod display;
mod format;
mod json;
mod records;

pub use args::{ApprovalArgs, ApprovalCommand, ApprovalDecisionArgs};

use anyhow::Result;
use codetether_approval_core::ApprovalStore;

/// Execute an operator-facing approval-store command.
///
/// # Arguments
/// * `args` - Selected operation and optional output or decision metadata.
/// # Returns
/// Unit after printing the result or recording a decision.
/// # Examples
/// ```no_run
/// use clap::Parser;
/// let args = codetether_approval_cli::ApprovalArgs::parse_from(["approval", "list", "--json"]);
/// codetether_approval_cli::execute(args)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// # Errors
///
/// Returns an error when the default approval store cannot be opened or the
/// requested approval operation fails.
pub fn execute(args: ApprovalArgs) -> Result<()> {
    let store = ApprovalStore::open_default()?;
    execute_with_store(args, &store)
}

fn execute_with_store(args: ApprovalArgs, store: &ApprovalStore) -> Result<()> {
    match args.command {
        ApprovalCommand::List { json } => display::list(store, json),
        ApprovalCommand::Show { id, json } => display::show(store, &id, json),
        ApprovalCommand::Approve(args) => decisions::approve(store, args),
        ApprovalCommand::Deny(args) => decisions::deny(store, args),
    }
}

#[cfg(test)]
mod tests;
