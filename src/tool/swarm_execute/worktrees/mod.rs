//! Worktree lifecycle support for `swarm_execute`.

mod create;
mod finish;
mod state;

pub(super) use state::SwarmWorktrees;
