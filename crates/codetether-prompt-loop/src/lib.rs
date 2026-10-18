//! Prompt-turn state and tool-call preparation independent of native sessions.
//!
//! [LoopState::new] resets per-turn counters and guards. [Call::new] normalizes
//! provider calls before [LoopState::blocked] checks them. [WorkspaceState]
//! tracks validation paths, and [StepFlow] describes the next loop action.
mod call;
mod flow;
mod guard;
mod new;
mod state;
mod workspace;
pub use call::Call;
pub use flow::StepFlow;
pub use state::LoopState;
pub use workspace::WorkspaceState;
