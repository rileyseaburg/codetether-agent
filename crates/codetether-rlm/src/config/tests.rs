//! Default configuration regressions.

use super::RlmConfig;

#[test]
fn default_history_trigger_is_disabled() {
    assert_eq!(RlmConfig::default().history_trigger_messages, 0);
}
