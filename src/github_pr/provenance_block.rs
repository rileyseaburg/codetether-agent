//! Generate provenance verification blocks for PR bodies.

use crate::provenance::{ExecutionProvenance, sign_provenance};

const SIG_PREVIEW_CHARS: usize = 16;

/// Build a markdown verification block from provenance metadata.
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
mod tests {
    use super::provenance_markdown_block;
    use crate::provenance::{ExecutionOrigin, ExecutionProvenance};

    #[test]
    fn renders_block_with_required_fields() {
        unsafe {
            std::env::remove_var("CODETETHER_SIGNING_SECRET");
        }
        let mut provenance = ExecutionProvenance::for_operation("ralph", ExecutionOrigin::Ralph);
        provenance.set_run_id("run-42");
        let block = provenance_markdown_block(&provenance);
        assert!(block.contains("### 🔐 Provenance"));
        assert!(block.contains("| Agent | `ralph` |"));
        assert!(block.contains("| Run ID | `run-42` |"));
        assert!(block.contains(&provenance.provenance_id));
        assert!(block.contains("`UNSIGNED`"));
    }
}
