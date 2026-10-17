//! Ordered evidence strings and capped changed-file references.
use super::ExecutionEvidence;
use crate::{ForageOpportunity, normalize_prompt_field};
pub(super) fn build(
    item: &ForageOpportunity,
    evidence: &ExecutionEvidence<'_>,
    cycle: usize,
    before_progress: f64,
    after_progress: f64,
) -> Vec<String> {
    let mut entries = vec![
        format!("cycle:{cycle}"),
        format!("engine:{}", evidence.engine),
        format!("model:{}", evidence.model.unwrap_or("default")),
        format!("score:{:.3}", item.score),
        format!("okr_id:{}", item.okr_id),
        format!("kr_id:{}", item.key_result_id),
        format!("progress_before_pct:{:.2}", before_progress * 100.0),
        format!("progress_after_pct:{:.2}", after_progress * 100.0),
        format!("detail:{}", normalize_prompt_field(&evidence.detail, 320)),
        format!(
            "concrete_file_evidence:{}",
            !evidence.changed_files.is_empty()
        ),
        format!("quality_gates_passed:{}", evidence.quality_gates_passed),
    ];
    let max_files = 40usize;
    for path in evidence.changed_files.iter().take(max_files) {
        entries.push(format!("file:{path}"));
    }
    if evidence.changed_files.len() > max_files {
        entries.push(format!(
            "files_truncated:{}",
            evidence.changed_files.len() - max_files
        ));
    }
    entries
}
