//! ResourceContent wire contract and construction.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceContent {
    Text { text: String },
    Blob { blob: String }, // base64-encoded
}
