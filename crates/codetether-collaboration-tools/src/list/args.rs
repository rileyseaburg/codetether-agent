//! Message or list arguments with trusted runtime context.
use crate::RuntimeContext;
use serde::Deserialize;

/// List prefix and trusted parent context.
///
/// # Examples
/// ```
/// let args:codetether_collaboration_tools::ListArgs=serde_json::from_value(serde_json::json!({})).unwrap();
/// assert!(args.path_prefix.is_none());
/// ```
#[derive(Deserialize)]
pub struct Args {
    /// Optional relative or canonical tree prefix.
    pub path_prefix: Option<String>,
    #[serde(flatten)]
    /// Trusted calling session identity.
    pub context: RuntimeContext,
}
