//! Default RLM runtime settings.

use super::{RlmConfig, config_defaults};

impl Default for RlmConfig {
    fn default() -> Self {
        use config_defaults::*;
        Self {
            mode: default_mode(),
            threshold: default_threshold(),
            max_iterations: default_max_iterations(),
            max_subcalls: default_max_subcalls(),
            runtime: default_runtime(),
            root_model: None,
            subcall_model: None,
            history_trigger_messages: default_history_trigger_messages(),
        }
    }
}
