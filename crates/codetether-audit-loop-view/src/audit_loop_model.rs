//! Data model for the audit-loop view.
//!
//! Represents the implement → audit → retry cycle: each iteration runs work,
//! then a set of audit gates (build, test, lint, file-limits) are checked.
//! The view lets the user watch progress instead of trusting a black box.

/// Outcome of a single audit gate within an iteration.
/// # Examples
/// ```
/// use codetether_audit_loop_view::audit_loop_model::GateStatus;
/// assert_eq!(GateStatus::Pending, GateStatus::Pending);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateStatus {
    /// Gate has not run yet.
    Pending,
    /// Gate is currently executing.
    Running,
    /// Gate passed.
    Passed,
    /// Gate failed; the loop should retry.
    Failed,
}

/// A single audit gate result (e.g. "cargo test", "file limits").
/// # Examples
/// ```
/// use codetether_audit_loop_view::audit_loop_model::{Gate, GateStatus};
/// let gate = Gate { name: "lint".into(), status: GateStatus::Pending, detail: None };
/// assert!(gate.detail.is_none());
/// ```
#[derive(Debug, Clone)]
pub struct Gate {
    /// Human-readable gate name.
    pub name: String,
    /// Current status.
    pub status: GateStatus,
    /// Optional short detail (error summary or pass note).
    pub detail: Option<String>,
}

/// Lifecycle status of a loop iteration.
/// # Examples
/// ```
/// use codetether_audit_loop_view::audit_loop_model::IterationStatus;
/// assert_eq!(IterationStatus::Running, IterationStatus::Running);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationStatus {
    /// Work/audit in progress.
    Running,
    /// All gates passed; loop can stop.
    Passed,
    /// One or more gates failed; a retry follows.
    Failed,
}

/// One pass through the implement→audit cycle.
/// # Examples
/// ```
/// use codetether_audit_loop_view::audit_loop_model::{Iteration, IterationStatus};
/// let iteration = Iteration { number: 1, summary: "first".into(), gates: vec![], status: IterationStatus::Running };
/// assert_eq!(iteration.number, 1);
/// ```
#[derive(Debug, Clone)]
pub struct Iteration {
    /// 1-based iteration number.
    pub number: usize,
    /// What this iteration attempted.
    pub summary: String,
    /// Gate results for this iteration.
    pub gates: Vec<Gate>,
    /// Overall iteration status.
    pub status: IterationStatus,
}
