//! Serializable provider/model capability records.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ModelCapability {
    pub provider: String,
    pub canonical_id: String,
    pub selectable_id: String,
    pub aliases: Vec<String>,
    pub qualifiers: Vec<String>,
    pub available: bool,
    pub source: &'static str,
}

#[derive(Debug, Serialize)]
pub struct ProviderCapability {
    pub provider: String,
    pub available: bool,
    pub source: &'static str,
    pub error: Option<String>,
    pub models: Vec<ModelCapability>,
}
