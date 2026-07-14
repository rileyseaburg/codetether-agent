//! Worktree lifecycle support for `swarm_execute`.

mod create;
mod finish;
mod integrate;
mod premerge;
mod state;

pub(super) use state::SwarmWorktrees;
