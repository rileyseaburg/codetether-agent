//! Harvest successful agent runs for SFT training data.

use anyhow::Result;
use std::path::Path;

/// Filter criteria for harvesting training records.
pub struct HarvestFilter {
    pub min_quality_score: f32,
    pub only_passed_gates: bool,
    pub max_records: usize,
}

impl Default for HarvestFilter {
    fn default() -> Self {
        Self {
            min_quality_score: 0.8,
            only_passed_gates: true,
            max_records: 1000,
        }
    }
}

/// Harvested training record.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrainingRecord {
    pub prompt: String,
    pub completion: String,
    pub quality_score: f32,
    pub source_run_id: String,
    pub tools_used: Vec<String>,
}

fn parse(line: &str, f: &HarvestFilter) -> Option<TrainingRecord> {
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

/// Harvest SFT records from the bus S3 sink (OpenAI-format JSONL).
pub fn harvest_from_dir(data_dir: &Path, filter: &HarvestFilter) -> Result<Vec<TrainingRecord>> {
    let mut out = Vec::new();
    let dir = data_dir.join("sft");
    if !dir.exists() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        if path.extension().map(|e| e != "jsonl").unwrap_or(true) {
            continue;
        }
        for line in std::fs::read_to_string(&path)?.lines() {
            if out.len() >= filter.max_records {
                break;
            }
            if let Some(r) = parse(line, filter) {
                out.push(r);
            }
        }
    }
    tracing::info!(records = out.len(), "Harvested training records");
    Ok(out)
}
