use super::level::EvidenceLevel;

const GUARDRAILS: &str = "\
Validation evidence rules:
- Label every verification claim as one of: {levels}.
- Do not say validated, passed, complete, proven, or deployed unless the same sentence names the validation level.
- Mocked routes, local Playwright, or fixture-backed tests must be called mocked local and cannot imply platform IDs or live upload success.
- Live Argo or platform claims require concrete evidence: namespace, app/job/pod name, sync or health state, artifact path, URL, or platform ID.
- A failed live validation is not completion when the user requested passing proof; continue with the next recovery run or name the blocker.
- If evidence artifacts were produced, preserve their paths in the final answer; do not remove them unless durable equivalent evidence is named.
- Let runtime tools and TetherScript do repeatable computation; summarize their concrete outputs instead of reasoning from memory.";

pub(crate) fn append_guardrails(system_prompt: String) -> String {
    if system_prompt.contains("Validation evidence rules:") {
        return system_prompt;
    }
    let live_claim = super::claim::assess_claim("Argo validation");
    let recovery = super::recovery::recovery_for(&live_claim, true);
    let guardrails = GUARDRAILS.replace("{levels}", &EvidenceLevel::all_labels());
    let ids = if live_claim.needs_concrete_id {
        format!(
            "\nRuntime classifier: {} claims need concrete IDs; recovery action: {recovery:?}.",
            live_claim.level.label(),
        )
    } else {
        String::new()
    };
    format!("{system_prompt}\n\n{guardrails}{ids}")
}

#[cfg(test)]
mod tests {
    use super::append_guardrails;

    #[test]
    fn injects_validation_level_terms() {
        let prompt = append_guardrails("base".to_string());
        assert!(prompt.contains("mocked local"));
        assert!(prompt.contains("live deployment/Argo"));
        assert!(prompt.contains("failed live validation is not completion"));
    }

    #[test]
    fn injection_is_idempotent() {
        let once = append_guardrails("base".to_string());
        let twice = append_guardrails(once.clone());
        assert_eq!(once, twice);
    }
}
