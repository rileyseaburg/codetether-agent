use serde::{Deserialize, Serialize};

/// Session settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    #[serde(default = "default_true")]
    pub auto_compact: bool,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: usize,
    #[serde(default = "default_true")]
    pub persist: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            auto_compact: true,
            max_tokens: default_max_tokens(),
            persist: true,
        }
    }
}

fn default_true() -> bool {
    true
}
fn default_max_tokens() -> usize {
    100_000
}
