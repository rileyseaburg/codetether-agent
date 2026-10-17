//! validation for PRD construction and validation.
use super::*;

pub(super) fn validate_prd(prd: &Prd) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if prd.project.is_empty() {
        errors.push("Missing required field: project".to_string());
    }
    if prd.feature.is_empty() {
        errors.push("Missing required field: feature".to_string());
    }
    if prd.user_stories.is_empty() {
        errors.push("PRD must have at least one user story".to_string());
    }

    let story_ids: HashSet<String> = prd.user_stories.iter().map(|s| s.id.clone()).collect();

    let mut seen_ids = HashSet::new();
    for story in &prd.user_stories {
        validate_story::validate(story, &story_ids, &mut seen_ids, &mut errors, &mut warnings);
    }

    if let Some(cycle) = detect_cycle(prd) {
        errors.push(format!(
            "Circular dependency detected: {}",
            cycle.join(" → ")
        ));
    }

    let qc = &prd.quality_checks;
    if qc.typecheck.is_none() && qc.test.is_none() && qc.lint.is_none() && qc.build.is_none() {
        warnings.push("No quality checks defined - ralph won't run repo-level gates".to_string());
    }

    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
    }
}
