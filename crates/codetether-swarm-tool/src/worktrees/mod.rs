//! Worktree lifecycle support for `swarm_execute`.

mod create;
mod dir;
mod finish;
mod integrate;
mod premerge;
mod state;

pub use state::SwarmWorktrees;
