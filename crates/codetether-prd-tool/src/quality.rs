//! quality for PRD generation and validation.
use super::*;
pub(super) fn convert(input: Option<QualityChecksInput>) -> QualityChecks {
    let quality_checks = match input {
        Some(qc) => QualityChecks {
            typecheck: qc.typecheck,
            test: qc.test,
            lint: qc.lint,
            build: qc.build,
        },
        None => QualityChecks::default(),
    };

    quality_checks
}
