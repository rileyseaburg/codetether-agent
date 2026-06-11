//! Dynamic tool API contract text.

pub(super) const DISCOVERY_ONLY_MODE: &str = "discovery_only";
pub(super) const DISCOVERY_ONLY_MESSAGE: &str =
    "Tool registered for discovery only. It is not available to agent execution.";

pub(super) fn is_agent_executable(mode: &str) -> bool {
    mode != DISCOVERY_ONLY_MODE
}

pub(super) fn registration_execution_mode() -> String {
    DISCOVERY_ONLY_MODE.to_string()
}

pub(super) fn discovery_registration_message() -> String {
    format!("{DISCOVERY_ONLY_MESSAGE} Heartbeat required every 30s.")
}

#[cfg(test)]
mod tests {
    use super::{
        DISCOVERY_ONLY_MODE, discovery_registration_message, is_agent_executable,
        registration_execution_mode,
    };

    #[test]
    fn discovery_tools_are_not_agent_executable() {
        assert!(!is_agent_executable(DISCOVERY_ONLY_MODE));
    }

    #[test]
    fn registered_tools_default_to_discovery_only_mode() {
        let mode = registration_execution_mode();

        assert_eq!(mode, DISCOVERY_ONLY_MODE);
        assert!(!is_agent_executable(&mode));
    }

    #[test]
    fn discovery_registration_message_mentions_heartbeat() {
        let message = discovery_registration_message();

        assert!(message.contains("discovery only"));
        assert!(message.contains("Heartbeat required every 30s"));
    }
}
