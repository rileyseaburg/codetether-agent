//! Operator-facing approval store commands.

mod args;
mod decisions;
mod defaults;
mod display;
mod format;
mod records;

pub use args::{ApprovalArgs, ApprovalCommand};

use crate::approval::ApprovalStore;
use anyhow::Result;

/// Execute `codetether approval`.
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
        ApprovalCommand::List => display::list(store),
        ApprovalCommand::Show { id } => display::show(store, &id),
        ApprovalCommand::Approve(args) => decisions::approve(store, args),
        ApprovalCommand::Deny(args) => decisions::deny(store, args),
    }
}

#[cfg(test)]
mod tests;
