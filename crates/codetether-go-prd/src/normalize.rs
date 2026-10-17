//! Set PRD timestamps and replace missing or mismatched quality checks.

use crate::quality::detect_quality_checks;
use codetether_ralph_core::Prd;
pub(crate) fn apply(prd: &mut Prd) {
    let now = chrono::Utc::now().to_rfc3339();
    prd.created_at = now.clone();
    prd.updated_at = now;

    // Normalize quality checks:
    // - If the model left them null, detect from the current directory.
    // - If the model guessed a toolchain that doesn't match the repo,
    //   override with detected checks (prevents cargo in Node/Go/Python repos).
    let cwd = std::env::current_dir().unwrap_or_default();
    let detected = detect_quality_checks();
    let needs_override = crate::mismatch::needs_override(prd, &cwd);

    if needs_override {
        prd.quality_checks = detected;
    }
}
