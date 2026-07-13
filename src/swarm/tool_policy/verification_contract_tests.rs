use super::VerificationContract;

#[test]
fn detects_preservation_without_verification_conflict() {
    let task = "Preserve every behavior. Do not run tests, builds, compilers, or linters.";
    assert_eq!(
        VerificationContract::from_instruction(task),
        VerificationContract::StaticOnly
    );
}

#[test]
fn requires_focused_verification_for_preservation() {
    let contract =
        VerificationContract::from_instruction("Preserve all behavior while refactoring");
    assert_eq!(contract, VerificationContract::FocusedRequired);
    assert!(contract.prompt().contains("requires focused verification"));
}

#[test]
fn honors_explicit_verification_waiver() {
    let task = "Preserve all behavior; verification waived by the user.";
    assert_eq!(
        VerificationContract::from_instruction(task),
        VerificationContract::Waived
    );
}

#[test]
fn calibrates_unsupported_preservation_summary() {
    let contract = VerificationContract::StaticOnly;
    let output = super::super::verification_output::calibrate(
        contract,
        "Refactor done.\nAll behavior preserved.",
    );
    assert!(output.contains("Static/local: source inspection only"));
    assert!(!output.contains("All behavior preserved"));
}

#[test]
fn leaves_unrelated_summary_unchanged() {
    let output = "Static/local: inspected module boundaries.";
    assert_eq!(
        super::super::verification_output::calibrate(VerificationContract::None, output),
        output
    );
}
