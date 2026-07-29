//! End-to-end check that `.tether` diagnostics reach the harness linter path.
//!
//! Guards the wiring rather than the language server itself: a syntax error in a
//! `.tether` file must surface through `LspManager::linter_diagnostics`, which is
//! the same call the post-edit validation hook makes in
//! `session::helper::validation`.
//!
//! Skips when the `tetherscript` binary is absent so CI without the toolchain
//! stays green.

use crate::lsp::LspManager;
use std::path::PathBuf;

/// Returns the repo-relative smoke fixture with a deliberate parse error.
fn broken_fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/tetherscript/_lsp_smoke_broken.tether")
}

#[tokio::test]
async fn tether_syntax_error_reaches_linter_diagnostics() {
    if which::which("tetherscript").is_err() {
        eprintln!("skipping: tetherscript binary not on PATH");
        return;
    }
    let path = broken_fixture();
    assert!(path.is_file(), "fixture missing at {}", path.display());

    let manager = LspManager::new(None);
    let diagnostics = manager.linter_diagnostics(&path).await;

    assert!(
        !diagnostics.is_empty(),
        "expected a parse diagnostic for {}",
        path.display()
    );
    let sources: Vec<_> = diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.source.as_deref())
        .collect();
    assert!(
        sources.contains(&"tetherscript"),
        "diagnostic should be attributed to tetherscript, got {sources:?}"
    );
}
