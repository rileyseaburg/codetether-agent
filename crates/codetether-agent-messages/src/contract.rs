//! Terminal deliverable contract injected into managed child prompts.
//! Terminal deliverable contract for durable managed agents.

/// Return the terminal status instructions injected into every managed child.
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// The unchanged static deliverable-status contract.
/// # Examples
/// ```rust
/// assert!(codetether_agent_messages::deliverable_contract().contains("STATUS: completed"));
/// ```
pub fn contract() -> &'static str {
    "DELIVERABLE STATUS CONTRACT:\n\
     - End the final response with exactly `STATUS: completed`, `STATUS: blocked`, or `STATUS: pending`.\n\
     - Completed requires concrete evidence.\n\
     - Blocked requires concrete blocker evidence.\n\
     - Pending requires the next action needed to continue.\n\
     - Never describe blocked or pending work as completed."
}
