//! Types shared between the search router, its backends, and the CLI.
//!
//! Keeping these in a dedicated module avoids circular deps and makes
//! backend plug-ins trivial to add later.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Identifier of a search backend the router can pick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    Grep,
    Glob,
    Websearch,
    Webfetch,
    Memory,
    Rlm,
}

impl Backend {
    /// Short identifier used in LLM prompts and JSON output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::search::types::Backend;
    /// assert_eq!(Backend::Grep.id(), "grep");
    /// ```
    pub fn id(self) -> &'static str {
        match self {
            Self::Grep => "grep",
            Self::Glob => "glob",
            Self::Websearch => "websearch",
            Self::Webfetch => "webfetch",
            Self::Memory => "memory",
            Self::Rlm => "rlm",
        }
    }
}

/// A single backend selection plus the args the router wants passed to it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendChoice {
    pub backend: Backend,
    #[serde(default)]
    pub args: Value,
}
