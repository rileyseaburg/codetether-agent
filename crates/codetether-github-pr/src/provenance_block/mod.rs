//! Generate provenance verification blocks for PR bodies.

use codetether_provenance::{ExecutionProvenance, sign_provenance};

const SIG_PREVIEW_CHARS: usize = 16;

/// Build a markdown verification block from provenance metadata.
///
/// # Arguments
/// * `provenance` - Execution identity and signing metadata.
/// # Returns
/// A Markdown table containing identity, origin, run, and signature preview.
/// # Examples
/// ```no_run
/// # fn example(provenance: &codetether_provenance::ExecutionProvenance) {
/// let block = codetether_github_pr::provenance_block::provenance_markdown_block(provenance);
/// assert!(block.contains("Provenance"));
/// # }
/// ```
pub fn provenance_markdown_block(provenance: &ExecutionProvenance) -> String {
    let signature = sign_provenance(provenance);
    let sig_label = signature.as_deref().unwrap_or("UNSIGNED");
    let sig_preview = &sig_label[..sig_label.len().min(SIG_PREVIEW_CHARS)];
    format!(
        "### 🔐 Provenance\n\
         \n\
         | Field | Value |\n\
         |-------|-------|\n\
         | Agent | `{agent}` |\n\
         | Origin | `{origin}` |\n\
         | Run ID | `{run}` |\n\
         | Provenance ID | `{prov}` |\n\
         | Signature | `{sig}` |\n\
         \n\
         > Verify: `codetether verify-pr {prov}`",
        agent = provenance.identity.agent_name,
        origin = provenance.identity.origin.as_str(),
        run = provenance.run_id.as_deref().unwrap_or("N/A"),
        prov = provenance.provenance_id,
        sig = sig_preview,
    )
}
#[cfg(test)]
mod tests;
