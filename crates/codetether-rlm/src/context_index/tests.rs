use super::{EvidenceKind, load_or_build, retrieve};

#[test]
fn retrieval_prefers_matching_symbols() {
    let src = "use crate::auth;\nfn login_user() {}\nfn render_home() {}";
    let index = load_or_build("auth.rs", src);
    let records = retrieve(&index, "login auth flow", 256);
    assert!(
        records
            .iter()
            .any(|r| r.symbols.iter().any(|s| s == "login_user"))
    );
}

#[test]
fn index_marks_error_evidence() {
    let index = load_or_build("run.log", "build failed: unresolved import");
    assert!(index.records.iter().any(|r| r.kind == EvidenceKind::Error));
}
