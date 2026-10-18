//! Classify the claimed evidence level and its concrete-ID requirement.
use super::level::EvidenceLevel;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Classified claim and whether it needs an external concrete identifier.
/// # Examples
/// ```
/// let claim = codetether_evidence_core::claim::assess_claim("Argo validation");
/// assert!(claim.needs_concrete_id);
/// ```
pub struct EvidenceClaim {
    /// Highest-priority matching evidence category.
    pub level: EvidenceLevel,
    /// Live deployment and platform claims require concrete IDs.
    pub needs_concrete_id: bool,
}

/// Classify a claim using case-insensitive marker precedence.
/// # Arguments
/// `text` is the claim to classify.
/// # Returns
/// Its inferred category and concrete-ID requirement; this does not verify proof.
/// # Examples
/// ```
/// assert!(codetether_evidence_core::claim::assess_claim("Argo validation").needs_concrete_id);
/// ```
pub fn assess_claim(text: &str) -> EvidenceClaim {
    let lower = text.to_ascii_lowercase();
    let level = if has_any(&lower, &["youtube", "tiktok", "platform id", "upload id"]) {
        EvidenceLevel::RealPlatformUpload
    } else if has_any(&lower, &["argo", "argocd", "kubectl", "namespace", "pod/"]) {
        EvidenceLevel::LiveDeploymentArgo
    } else if has_any(&lower, &["ci-like", "playwright", "e2e", "focused run"]) {
        EvidenceLevel::FocusedCiLike
    } else if has_any(&lower, &["mock", "fixture", "route.fulfill"]) {
        EvidenceLevel::MockedLocal
    } else if has_any(&lower, &["fmt", "clippy", "test", "lint", "diff --check"]) {
        EvidenceLevel::StaticLocal
    } else {
        EvidenceLevel::NotRun
    };
    EvidenceClaim {
        level,
        needs_concrete_id: matches!(
            level,
            EvidenceLevel::LiveDeploymentArgo | EvidenceLevel::RealPlatformUpload
        ),
    }
}

fn has_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::{EvidenceLevel, assess_claim};

    #[test]
    fn argo_claims_need_concrete_ids() {
        let claim = assess_claim("Argo CD pod passed");
        assert_eq!(claim.level, EvidenceLevel::LiveDeploymentArgo);
        assert!(claim.needs_concrete_id);
    }
}
