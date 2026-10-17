//! Types shared between the search router, its backends, and the CLI.
//!
//! Keeping these in a dedicated module avoids circular deps and makes
//! backend plug-ins trivial to add later.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Identifier of a search backend the router can pick.
///
/// Variants cover workspace text/files, web search/fetch, memory, and RLM.
/// # Examples
/// ```
/// use codetether_search::Backend;
/// let kind = match Backend::Grep {
///     Backend::Grep | Backend::Glob => "workspace",
///     Backend::Websearch | Backend::Webfetch => "web",
///     Backend::Memory => "memory",
///     Backend::Rlm => "semantic",
/// };
/// assert_eq!(kind, "workspace");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    /// Search workspace text by pattern.
    Grep,
    /// Search workspace filenames by glob.
    Glob,
    /// Search web results.
    Websearch,
    /// Retrieve a web page.
    Webfetch,
    /// Search stored memory.
    Memory,
    /// Request recursive language-model analysis.
    Rlm,
}

impl Backend {
    /// Short identifier used in LLM prompts and JSON output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_search::types::Backend;
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
///
/// # Examples
/// ```
/// use codetether_search::{Backend, BackendChoice};
/// let choice = BackendChoice { backend: Backend::Grep, args: serde_json::json!({}) };
/// assert_eq!(choice.backend.id(), "grep");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendChoice {
    /// Backend chosen by the model.
    pub backend: Backend,
    /// Proposed backend arguments before normalization.
    #[serde(default)]
    pub args: Value,
}
