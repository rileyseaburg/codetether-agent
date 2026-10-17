//! Detect generated typecheck commands that do not match project marker files.

use codetether_ralph_core::Prd;
use std::path::Path;
pub(crate) fn needs_override(prd: &Prd, cwd: &Path) -> bool {
    let looks_like_cargo = prd
        .quality_checks
        .typecheck
        .as_deref()
        .map(|c| c.to_ascii_lowercase().contains("cargo"))
        .unwrap_or(false);
    let looks_like_npm = prd
        .quality_checks
        .typecheck
        .as_deref()
        .map(|c| {
            let c = c.to_ascii_lowercase();
            c.contains("npm") || c.contains("pnpm") || c.contains("yarn") || c.contains("npx")
        })
        .unwrap_or(false);
    let looks_like_go = prd
        .quality_checks
        .typecheck
        .as_deref()
        .map(|c| c.to_ascii_lowercase().contains("go vet"))
        .unwrap_or(false);

    // Override if: no typecheck set, OR guessed tool doesn't match project files
    let needs_override = prd.quality_checks.typecheck.is_none()
        || (looks_like_cargo && !cwd.join("Cargo.toml").exists())
        || (looks_like_npm && !cwd.join("package.json").exists())
        || (looks_like_go && !cwd.join("go.mod").exists());

    needs_override
}
