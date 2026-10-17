//! Extended error envelope schema.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct OpenRouterError {
    pub(crate) error: OpenRouterErrorDetail,
}
#[derive(Debug, Deserialize)]
pub(crate) struct OpenRouterErrorDetail {
    pub(crate) message: String,
    #[serde(default)]
    pub(crate) code: Option<Value>,
    #[serde(default)]
    pub(crate) metadata: Option<Value>,
}
