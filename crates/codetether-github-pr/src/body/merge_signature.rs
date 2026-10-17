//! PR body merge signature.
use super::*;

/// Append the execution signature, replacing an existing signature block.
///
/// # Arguments
/// * `base` - Existing PR body.
/// * `signature` - Metadata to render between the signature markers.
/// # Returns
/// Combined body with surrounding text retained.
/// # Examples
/// ```
/// # fn example(signature: &codetether_github_pr::context::SignatureContext) {
/// let body = codetether_github_pr::body::merge_signature("summary", signature);
/// assert!(body.starts_with("summary"));
/// # }
/// ```
pub fn merge_signature(base: &str, signature: &SignatureContext) -> String {
    let block = render_signature(signature);
    if let Some((start, end)) = signature_range(base) {
        let mut merged = String::new();
        merged.push_str(base[..start].trim_end());
        if !merged.is_empty() {
            merged.push_str("\n\n");
        }
        merged.push_str(&block);
        let tail = base[end..].trim();
        if !tail.is_empty() {
            merged.push_str("\n\n");
            merged.push_str(tail);
        }
        return merged;
    }
    if base.trim().is_empty() {
        return block;
    }
    format!("{}\n\n{}", base.trim_end(), block)
}
