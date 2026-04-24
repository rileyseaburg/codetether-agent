//! Dynamic tool API contract text.

pub(super) const DISCOVERY_ONLY_MODE: &str = "discovery_only";
pub(super) const DISCOVERY_ONLY_MESSAGE: &str =
    "Tool registered for discovery only. It is not available to agent execution.";

pub(super) fn is_agent_executable(mode: &str) -> bool {
    mode != DISCOVERY_ONLY_MODE
}

#[cfg(test)]
mod tests {
    use super::{DISCOVERY_ONLY_MODE, is_agent_executable};

    #[test]
    fn discovery_tools_are_not_agent_executable() {
        assert!(!is_agent_executable(DISCOVERY_ONLY_MODE));
    }
}
