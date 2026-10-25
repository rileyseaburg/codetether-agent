//! # Worktree Delivery
//!
//! Delivers an isolated worker branch into the checkout that spawned it.
//!
//! Delivery applies the branch's net patch to the target checkout's working
//! tree as **uncommitted edits**. It never commits, merges, stages, stashes,
//! or moves `HEAD` in the target, so the user's Git history and index stay
//! under the user's control. Application is all-or-nothing: a patch that does
//! not apply cleanly leaves the target untouched and reports why.
//!
//! Every attempt is recorded, with its full patch, in the target workspace's
//! SQLite delivery ledger (see [`ledger_path`]), so delivered work remains
//! recoverable after the worker checkout is removed.
//!
//! ## Usage
//!
//! ```rust,no_run
//! # async fn example(manager: &codetether_worktree::WorktreeManager) -> anyhow::Result<()> {
//! let delivery = manager.deliver("swarm_task").await?;
//! println!("{}", delivery.summary());
//! # Ok(()) }
//! ```

mod apply;
mod connection;
mod discard;
mod git;
mod ledger;
mod outcome;
mod plan;
mod run;
mod schema;
#[cfg(test)]
mod tests;

pub use connection::ledger_path;
pub use outcome::{Delivery, DeliveryStatus};

use crate::WorktreeManager;
use anyhow::{Context, Result};
use std::path::Path;

impl WorktreeManager {
    /// Deliver a managed worktree's branch into this manager's checkout.
    ///
    /// The target is the checkout this manager was created for, which is the
    /// checkout the worker was spawned from.
    ///
    /// # Errors
    ///
    /// Returns an error if the worktree is unknown, Git cannot compute the
    /// patch, or the SQLite ledger cannot record the attempt. A patch that
    /// does not apply is not an error; it yields [`DeliveryStatus::Rejected`].
    pub async fn deliver(&self, name: &str) -> Result<Delivery> {
        let target = self.repo_path.clone();
        self.deliver_to(name, &target).await
    }

    /// Deliver a managed worktree's branch into `target` as uncommitted edits.
    ///
    /// # Arguments
    ///
    /// * `name` — Managed worktree name whose branch is delivered.
    /// * `target` — Any path inside the receiving checkout of the same repository.
    ///
    /// # Errors
    ///
    /// Same conditions as [`WorktreeManager::deliver`].
    pub async fn deliver_to(&self, name: &str, target: &Path) -> Result<Delivery> {
        self.validate_storage()?;
        let branch = self.branch_for(name).await?;
        deliver_branch(&branch, target).await
    }
}

/// Deliver any local `branch` into the checkout containing `target`.
///
/// Same semantics as [`WorktreeManager::deliver`]: uncommitted edits only,
/// all-or-nothing, recorded in the SQLite delivery ledger.
///
/// # Errors
///
/// Returns an error if Git cannot compute the patch or the ledger cannot
/// record the attempt.
pub async fn deliver_branch(branch: &str, target: &Path) -> Result<Delivery> {
    let branch = branch.to_string();
    let target = target.to_path_buf();
    tokio::task::spawn_blocking(move || run::deliver(&branch, &target))
        .await
        .context("worktree delivery task failed")?
}
