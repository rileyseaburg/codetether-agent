//! signatures response schema.
use super::*;
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ExtraContent {
    pub(crate) google: Option<GoogleExtra>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct GoogleExtra {
    pub(crate) thought_signature: Option<String>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct FunctionCall {
    pub(crate) name: String,
    pub(crate) arguments: String,
}
