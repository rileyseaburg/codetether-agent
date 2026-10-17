//! JSON parser for the router LLM response.
//!
//! Accepts raw text, strips any accidental fencing, and extracts the
//! first JSON object we can parse into a [`RouterPlan`].

use anyhow::{Context, Result, anyhow};
use serde::Deserialize;

use super::types::BackendChoice;

/// Parsed plan returned by the router model.
///
/// # Examples
/// ```
/// use codetether_search::parse::RouterPlan;
/// let plan = RouterPlan { choices: vec![] };
/// assert!(plan.choices.is_empty());
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct RouterPlan {
    /// Backend choices in the model's preferred order.
    pub choices: Vec<BackendChoice>,
}

/// Parse a router response string into a [`RouterPlan`].
///
/// # Arguments
/// * `raw` - Raw model text, optionally wrapped in prose or code fences.
/// # Returns
/// The decoded backend choices.
///
/// # Errors
///
/// Returns an error when no JSON object is present or the payload fails
/// to match the expected schema.
///
/// # Examples
///
/// ```rust
/// use codetether_search::parse::parse_router_response;
/// let plan = parse_router_response(
///     r#"```json
///     {"choices":[{"backend":"grep","args":{"pattern":"fn main"}}]}
///     ```"#,
/// )
/// .unwrap();
/// assert_eq!(plan.choices.len(), 1);
/// ```
pub fn parse_router_response(raw: &str) -> Result<RouterPlan> {
    let trimmed = raw.trim();
    if let Ok(plan) = serde_json::from_str::<RouterPlan>(trimmed) {
        return Ok(plan);
    }
    let (start, end) = (
        trimmed.find('{').ok_or_else(|| anyhow!("no '{{' found"))?,
        trimmed.rfind('}').ok_or_else(|| anyhow!("no '}}' found"))?,
    );
    serde_json::from_str(&trimmed[start..=end]).context("router response is not valid RouterPlan")
}
