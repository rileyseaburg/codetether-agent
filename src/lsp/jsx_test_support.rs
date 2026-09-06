//! Availability and diagnostic assertions for real JSX language-server tests.

use super::LspActionResult;

pub(super) fn available() -> bool {
    std::process::Command::new("typescript-language-server")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

pub(super) fn assert_no_errors(result: LspActionResult) {
    let LspActionResult::Diagnostics { diagnostics } = result else {
        panic!("diagnostics expected")
    };
    let errors: Vec<_> = diagnostics
        .into_iter()
        .filter(|diagnostic| diagnostic.severity.as_deref() == Some("error"))
        .collect();
    assert!(errors.is_empty(), "unexpected JSX diagnostics: {errors:?}");
}
