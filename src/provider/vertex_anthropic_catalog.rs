//! Static model catalog for the Vertex AI Anthropic provider.
//!
//! Single responsibility: turn the row table in
//! [`super::vertex_anthropic_rows`] into [`ModelInfo`] values. Row
//! construction lives in [`super::vertex_anthropic_model`].
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::vertex_anthropic::vertex_anthropic_catalog::catalog;
//!
//! let models = catalog();
//! let opus5 = models.iter().find(|m| m.id == "claude-opus-5").unwrap();
//! assert_eq!(opus5.context_window, 1_000_000);
//! ```

use super::super::ModelInfo;
use super::vertex_anthropic_model::entry;
use super::vertex_anthropic_rows::ROWS;

/// Build the Vertex AI Anthropic model catalog, newest model first.
///
/// # Returns
///
/// Owned [`ModelInfo`] rows for `Provider::list_models`.
pub fn catalog() -> Vec<ModelInfo> {
    ROWS.iter()
        .map(|&(id, name, ctx, out, cin, cout)| entry(id, name, ctx, out, cin, cout))
        .collect()
}

#[cfg(test)]
#[path = "vertex_anthropic_catalog_tests.rs"]
mod tests;
