//! Alias dispatch for newer Claude families (Opus and Sonnet 5.x).
//!
//! Keeps the main [`super::resolve_model_id`] table from growing each time a
//! new Claude generation ships.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::bedrock::resolve_model_id;
//!
//! assert_eq!(resolve_model_id("claude-opus-5"), "global.anthropic.claude-opus-5");
//! assert_eq!(
//!     resolve_model_id("us.anthropic.claude-sonnet-5-5"),
//!     "global.anthropic.claude-sonnet-5-5"
//! );
//! ```

#[path = "aliases_sonnet5.rs"]
mod sonnet5;

/// Resolve an Opus or Sonnet 5.x alias, or `None` for other families.
pub(super) fn resolve(model: &str) -> Option<&'static str> {
    super::aliases_opus::resolve_opus_alias(model).or_else(|| sonnet5::resolve_sonnet5_alias(model))
}
