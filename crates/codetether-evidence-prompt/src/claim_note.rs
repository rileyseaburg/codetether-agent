//! Render the example runtime classifier and recovery note.
/// Render the example runtime classifier and recovery note.
/// # Arguments
/// This renderer takes no arguments.
/// # Returns
/// The established instruction text, with its original labels and ordering.
/// # Examples
/// ```
/// let text = codetether_evidence_prompt::claim_note::render();
/// assert!(!text.is_empty());
/// ```
pub fn render() -> String {
    let live_claim = codetether_evidence_core::claim::assess_claim("Argo validation");
    if !live_claim.needs_concrete_id {
        return String::new();
    }
    let recovery = codetether_evidence_core::recovery::recovery_for(&live_claim, true);
    let workflow = codetether_evidence_core::workflow::infer("Argo platform upload");
    format!(
        "\nRuntime classifier: {} claims need concrete IDs; recovery action: {recovery:?}; gate: {}.",
        live_claim.level.label(),
        workflow.required_gate(),
    )
}
