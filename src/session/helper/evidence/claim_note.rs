pub(crate) fn render() -> String {
    let live_claim = super::claim::assess_claim("Argo validation");
    if !live_claim.needs_concrete_id {
        return String::new();
    }
    let recovery = super::recovery::recovery_for(&live_claim, true);
    let workflow = super::workflow::infer("Argo platform upload");
    format!(
        "\nRuntime classifier: {} claims need concrete IDs; recovery action: {recovery:?}; gate: {}.",
        live_claim.level.label(),
        workflow.required_gate(),
    )
}
