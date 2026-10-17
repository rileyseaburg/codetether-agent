//! Task slot reservation outcomes.

/// Result of attempting to reserve a task-processing slot.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::dispatch::TaskReservation;
/// let outcome = TaskReservation::Reserved;
/// let label = match outcome {
///     TaskReservation::Reserved => "reserved",
///     TaskReservation::AlreadyProcessing => "active",
///     TaskReservation::AtCapacity => "full",
/// };
/// assert_eq!(label, "reserved");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskReservation {
    /// The task ID was inserted into the active set.
    Reserved,
    /// This task ID already has an active reservation.
    AlreadyProcessing,
    /// The active set has reached the configured limit.
    AtCapacity,
}
