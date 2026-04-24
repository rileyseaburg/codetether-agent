use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct KilnPluginInput {
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    pub hook: String,
    #[serde(default)]
    pub args: Vec<Value>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

impl KilnPluginInput {
    pub fn has_source(&self) -> bool {
        self.source
            .as_deref()
            .is_some_and(|source| !source.is_empty())
    }

    pub fn has_path(&self) -> bool {
        self.path.as_deref().is_some_and(|path| !path.is_empty())
    }
}
