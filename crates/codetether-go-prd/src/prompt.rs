//! Build a task-specific PRD generation prompt from objective key results.

use codetether_okr::Okr;
pub(crate) fn build(task: &str, okr: &Okr) -> String {
    let kr_descriptions: Vec<String> = okr
        .key_results
        .iter()
        .enumerate()
        .map(|(i, kr)| {
            format!(
                "KR-{}: {} (target: {} {})",
                i + 1,
                kr.title,
                kr.target_value,
                kr.unit
            )
        })
        .collect();

    format!(
        include_str!("prompt.md"),
        task = task,
        krs = kr_descriptions.join("\n"),
    )
}
