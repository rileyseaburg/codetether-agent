//! Phrase detection for delegation verification requirements.

pub(super) fn preserves_behavior(text: &str) -> bool {
    [
        "preserve all behavior",
        "preserve every behavior",
        "behavioral equivalence",
        "all behavior preserved",
        "every behavior preserved",
    ]
    .iter()
    .any(|phrase| text.contains(phrase))
}

pub(super) fn prohibits_verification(text: &str) -> bool {
    (text.contains("do not run") || text.contains("no "))
        && ["test", "build", "compiler", "lint"]
            .iter()
            .any(|term| text.contains(term))
}

pub(super) fn waived(text: &str) -> bool {
    text.contains("verification waived") || text.contains("waive verification")
}
