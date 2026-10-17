//! parse uri operations for language server clients.

use anyhow::Result;

/// Parses a URI for use in LSP request parameters.
///
/// # Arguments
/// * `uri_str` - URI text accepted by the LSP URI parser.
///
/// # Returns
/// The parsed URI.
///
/// # Errors
/// Returns an error prefixed with `Invalid URI:` if parsing fails.
///
/// # Examples
/// ```
/// let uri = codetether_lsp_core::uri::parse_uri("file:///tmp/example.rs").unwrap();
/// assert_eq!(uri.as_str(), "file:///tmp/example.rs");
/// ```
pub fn parse_uri(uri_str: &str) -> Result<lsp_types::Uri> {
    uri_str
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid URI: {e}"))
}
