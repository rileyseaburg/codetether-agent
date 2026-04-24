//! Dynamic tool API contract text.

pub(super) const DISCOVERY_ONLY_MODE: &str = "discovery_only";
pub(super) const DISCOVERY_ONLY_MESSAGE: &str =
    "Tool registered for discovery only. It is not available to agent execution.";

pub(super) fn is_agent_executable(mode: &str) -> bool {
    mode != DISCOVERY_ONLY_MODE
}

pub(super) fn discovery_registration_message() -> String {
    format!("{DISCOVERY_ONLY_MESSAGE} Heartbeat required every 30s.")
}

#[cfg(test)]
mod tests {
    use super::{DISCOVERY_ONLY_MODE, discovery_registration_message, is_agent_executable};

    #[test]
    fn discovery_tools_are_not_agent_executable() {
        assert!(!is_agent_executable(DISCOVERY_ONLY_MODE));
    }

    #[test]
    fn discovery_registration_message_mentions_heartbeat() {
        let message = discovery_registration_message();

        assert!(message.contains("discovery only"));
        assert!(message.contains("Heartbeat required every 30s"));
    }
}
