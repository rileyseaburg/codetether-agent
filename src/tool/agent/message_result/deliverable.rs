//! Terminal deliverable contract for durable managed agents.

/// Return the terminal status instructions injected into every managed child.
pub(in crate::tool::agent) fn contract() -> &'static str {
    "DELIVERABLE STATUS CONTRACT:\n\
     - End the final response with exactly `STATUS: completed`, `STATUS: blocked`, or `STATUS: pending`.\n\
     - Completed requires concrete evidence.\n\
     - Blocked requires concrete blocker evidence.\n\
     - Pending requires the next action needed to continue.\n\
     - Never describe blocked or pending work as completed."
}

/// Preserve runtime errors or reject a response without a completed status.
pub(in crate::tool::agent) fn effective_error(
    response: &str,
    error: Option<String>,
) -> Option<String> {
    error.or_else(|| crate::swarm::tool_policy::deliverable_error(response))
}

#[cfg(test)]
mod tests {
    use super::effective_error;

    #[test]
    fn blocked_and_pending_are_failures() {
        assert!(effective_error("evidence\nSTATUS: blocked", None).is_some());
        assert!(effective_error("next action\nSTATUS: pending", None).is_some());
        assert!(effective_error("evidence\nSTATUS: completed", None).is_none());
    }
}
