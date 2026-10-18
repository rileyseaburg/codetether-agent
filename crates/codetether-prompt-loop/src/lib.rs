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
mod runner_state;
pub use runner_state::RunnerState;

/// Provider-response retry decisions and corrective messages.
pub mod response;

/// Assistant stream events, bus publication and final-answer accumulation.
pub mod output;

/// Ordered transcript repair after truncated provider tool calls.
pub mod repair;

/// Sequencing of response guards, publication, repair and tool execution.
pub mod driver;

mod reset;
/// Terminal response validation and continuation sequencing.
pub mod terminal;

/// Policy and effect sequencing for one normalized tool call.
pub mod tool_driver;

/// Completed and synthetic tool-result publication sequencing.
pub mod tool_publication;

/// Whole-turn continuation after exhausted provider failures.
pub mod goal_recovery;

/// Provider completion attempt state and retry sequencing.
pub mod completion;

/// Provider metrics, goal consumption and live usage reporting.
pub mod usage;

/// Iterative provider/tool lifecycle and step-budget decisions.
pub mod lifecycle;

/// Human text/image input and title-generation sequencing.
pub mod input;

/// Ordered model, workspace, and progress initialization.
pub mod setup;
