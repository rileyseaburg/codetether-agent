use super::level::EvidenceLevel;

const TEMPLATE: &str = "\
Validation evidence rules:
- Label every verification claim as one of: {levels}.
- Do not say validated, passed, complete, proven, or deployed unless the same sentence names the validation level.
- Mocked routes, local Playwright, or fixture-backed tests must be called mocked local and cannot imply platform IDs or live upload success.
- Live Argo or platform claims require concrete evidence: namespace, app/job/pod name, sync or health state, artifact path, URL, or platform ID.
- A failed live validation is not completion when the user requested passing proof; continue with the next recovery run or name the blocker.
- Do not collapse an all-encompassing task into one proven slice; report each deliverable as {scope_statuses}.
- If evidence artifacts were produced, preserve their paths in the final answer; do not remove them unless durable equivalent evidence is named.
- Let runtime tools and TetherScript do repeatable computation; summarize their concrete outputs instead of reasoning from memory.";

pub(crate) fn render() -> String {
    TEMPLATE
        .replace("{levels}", &EvidenceLevel::all_labels())
        .replace("{scope_statuses}", &super::scope::status_labels())
}
