//! Decode one record and apply the harvesting policy.
use super::{HarvestFilter, TrainingRecord};
pub(super) fn parse(line: &str, f: &HarvestFilter) -> Option<TrainingRecord> {
    let v: serde_json::Value = serde_json::from_str(line).ok()?;
    let score = v
        .get("quality_score")
        .and_then(|x| x.as_f64())
        .unwrap_or(0.0) as f32;
    let passed = v
        .get("all_gates_passed")
        .and_then(|x| x.as_bool())
        .unwrap_or(false);
    if score < f.min_quality_score || (f.only_passed_gates && !passed) {
        return None;
    }
    let tools_used = v
        .get("tools_used")
        .and_then(|x| x.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let s = |k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    Some(TrainingRecord {
        prompt: s("prompt"),
        completion: s("completion"),
        quality_score: score,
        source_run_id: s("run_id"),
        tools_used,
    })
}
