//! PRD response recovery: detect quality checks.

use codetether_ralph_core::QualityChecks;

/// Auto-detect quality checks from the working directory.
pub(crate) fn detect_quality_checks() -> QualityChecks {
    let cwd = std::env::current_dir().unwrap_or_default();

    if cwd.join("Cargo.toml").exists() {
        QualityChecks {
            typecheck: Some("cargo check".to_string()),
            test: Some("cargo test".to_string()),
            lint: Some("cargo clippy --all-features".to_string()),
            build: Some("cargo build".to_string()),
        }
    } else if cwd.join("package.json").exists() {
        QualityChecks {
            typecheck: Some("npx tsc --noEmit".to_string()),
            test: Some("npm test".to_string()),
            lint: Some("npm run lint".to_string()),
            build: Some("npm run build".to_string()),
        }
    } else if cwd.join("go.mod").exists() {
        QualityChecks {
            typecheck: Some("go vet ./...".to_string()),
            test: Some("go test ./...".to_string()),
            lint: Some("golangci-lint run".to_string()),
            build: Some("go build ./...".to_string()),
        }
    } else if cwd.join("requirements.txt").exists() || cwd.join("pyproject.toml").exists() {
        QualityChecks {
            typecheck: Some("mypy .".to_string()),
            test: Some("pytest".to_string()),
            lint: Some("ruff check .".to_string()),
            build: None,
        }
    } else {
        QualityChecks::default()
    }
}
