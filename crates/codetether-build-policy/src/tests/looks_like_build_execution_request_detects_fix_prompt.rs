//! Original build-policy regression case.
use crate::*;

#[test]
fn looks_like_build_execution_request_detects_fix_prompt() {
    assert!(looks_like_build_execution_request(
        "yes fix it and patch src/tool/lsp.rs"
    ));
    assert!(!looks_like_build_execution_request(
        "what does this module do?"
    ));
}
