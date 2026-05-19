use serde::Deserialize;
use serde_json::Value;

/// Input parameters for the `tetherscript_plugin` tool.
///
/// Browser capability fields (`grant_browser`, `browser_origin`, `browser_scope`)
/// are optional. When `grant_browser` is provided the runner creates a
/// `BrowserAuthority` and grants it to the plugin host.
#[derive(Debug, Deserialize)]
pub struct TetherScriptPluginInput {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    pub hook: String,
    #[serde(default)]
    pub args: Vec<Value>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    /// Browser bridge endpoint, e.g. `http://127.0.0.1:41707/browser`.
    #[serde(default)]
    pub grant_browser: Option<String>,
    /// Allowed origins for browser capability.
    #[serde(default)]
    pub browser_origin: Vec<String>,
    /// Allowed scopes for browser capability.
    #[serde(default)]
    pub browser_scope: Vec<String>,
}

impl TetherScriptPluginInput {
    pub fn has_source(&self) -> bool {
        self.source
            .as_deref()
            .is_some_and(|source| !source.is_empty())
    }

    pub fn has_path(&self) -> bool {
        self.path.as_deref().is_some_and(|path| !path.is_empty())
    }

    /// Whether browser capability should be granted to the plugin.
    #[cfg(test)]
    pub fn wants_browser(&self) -> bool {
        self.grant_browser
            .as_deref()
            .is_some_and(|ep| !ep.is_empty())
    }
}
