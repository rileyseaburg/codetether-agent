use serde_json::Value;

use crate::tool::tetherscript::input::TetherScriptPluginInput;

use super::constants::{DEFAULT_TIMEOUT_SECS, MAX_TIMEOUT_SECS};

/// Parameters for a TetherScript execution request.
pub struct TetherScriptRun {
    pub source_name: String,
    pub source: String,
    pub hook: String,
    pub args: Vec<Value>,
    pub timeout_secs: u64,
    pub grant_browser: Option<String>,
    pub browser_origin: Vec<String>,
    pub browser_scope: Vec<String>,
}

impl TetherScriptRun {
    /// Build a run request from loaded source and parsed input.
    pub fn new(source_name: String, source: String, input: TetherScriptPluginInput) -> Self {
        Self {
            source_name,
            source,
            hook: input.hook,
            args: input.args,
            timeout_secs: input
                .timeout_secs
                .unwrap_or(DEFAULT_TIMEOUT_SECS)
                .min(MAX_TIMEOUT_SECS),
            grant_browser: input.grant_browser,
            browser_origin: input.browser_origin,
            browser_scope: input.browser_scope,
        }
    }
}
