//! Serialization default values.

pub fn default_mode() -> String {
    "auto".into()
}

pub fn default_threshold() -> f64 {
    0.35
}

pub fn default_max_iterations() -> usize {
    15
}

pub fn default_max_subcalls() -> usize {
    50
}

pub fn default_runtime() -> String {
    "rust".into()
}

pub fn default_history_trigger_messages() -> usize {
    0
}
