//! Events emitted by an audit loop, consumed by the view state.

use super::audit_loop_model::{Gate, GateStatus, Iteration, IterationStatus};

/// An event describing progress of the implement→audit→retry loop.
#[derive(Debug, Clone)]
pub enum AuditLoopEvent {
    /// Loop started with a task label and a retry cap.
    Started { task: String, max_iterations: usize },
    /// A new iteration began.
    IterationStarted { number: usize, summary: String },
    /// A gate began running in the current iteration.
    GateStarted { name: String },
    /// A gate finished with a pass/fail result and optional detail.
    GateFinished {
        name: String,
        passed: bool,
        detail: Option<String>,
    },
    /// The current iteration finished (all gates resolved).
    IterationFinished { passed: bool },
    /// The whole loop ended.
    Complete { passed: bool, iterations: usize },
}

/// Build a fresh gate in the pending state.
pub(super) fn new_gate(name: String) -> Gate {
    Gate {
        name,
        status: GateStatus::Pending,
        detail: None,
    }
}

/// Build a fresh running iteration.
pub(super) fn new_iteration(number: usize, summary: String) -> Iteration {
    Iteration {
        number,
        summary,
        gates: Vec::new(),
        status: IterationStatus::Running,
    }
}
