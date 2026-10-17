use super::provenance_markdown_block;
use codetether_provenance::{ExecutionOrigin, ExecutionProvenance};

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
