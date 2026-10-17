//! Global sub-agent settings plus named agent profile definitions.

use super::AgentConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};

const DEFAULT_MAX_THREADS: usize = 6;
const DEFAULT_MAX_DEPTH: u8 = 1;

/// Configuration represented by the Codex-compatible `[agents]` table.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentsConfig {
    /// Maximum concurrent child threads; `max_threads` is the Codex alias.
    #[serde(default, alias = "max_threads")]
    pub max_concurrent_threads_per_session: Option<NonZeroUsize>,
    /// Maximum spawned nesting depth, with the root session at depth zero.
    #[serde(default)]
    pub max_depth: Option<u8>,
    /// Whether interrupted turns receive a model-visible history marker.
    #[serde(default)]
    pub interrupt_message: Option<bool>,
    /// Named agent profiles represented by nested `[agents.<name>]` tables.
    #[serde(flatten)]
    pub profiles: HashMap<String, AgentConfig>,
}

impl AgentsConfig {
    /// Return the child-thread limit, which defaults to six.
    pub fn max_threads(&self) -> usize {
        self.max_concurrent_threads_per_session
            .map(NonZeroUsize::get)
            .unwrap_or(DEFAULT_MAX_THREADS)
    }

    /// Return the nesting-depth limit, which defaults to one.
    pub fn max_depth(&self) -> u8 {
        self.max_depth.unwrap_or(DEFAULT_MAX_DEPTH)
    }

    /// Return the effective interruption-marker setting, which defaults true.
    pub fn interrupt_message_enabled(&self) -> bool {
        self.interrupt_message.unwrap_or(true)
    }
}

impl Deref for AgentsConfig {
    type Target = HashMap<String, AgentConfig>;

    fn deref(&self) -> &Self::Target {
        &self.profiles
    }
}

impl DerefMut for AgentsConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.profiles
    }
}
