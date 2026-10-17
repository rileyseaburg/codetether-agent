//! Harvest filtering and filesystem contract.
mod support;
use codetether_distill::harvest::{HarvestFilter, harvest_from_dir};
use support::Directory;

#[test]
fn filters_records_and_preserves_defaults_and_limit() {
    let dir = Directory::new();
    std::fs::create_dir(dir.0.join("sft")).unwrap();
    let rows = [
        "malformed",
        r#"{"quality_score":0.9,"all_gates_passed":false}"#,
        r#"{"quality_score":0.7,"all_gates_passed":true}"#,
        r#"{"quality_score":0.8,"all_gates_passed":true,"run_id":"run","tools_used":["read",4,"write"]}"#,
        r#"{"quality_score":1.0,"all_gates_passed":true}"#,
    ];
    std::fs::write(dir.0.join("sft/runs.jsonl"), rows.join("\n")).unwrap();
    std::fs::write(dir.0.join("sft/ignored.txt"), rows[4]).unwrap();
    let filter = HarvestFilter {
        max_records: 1,
        ..Default::default()
    };
    let records = harvest_from_dir(&dir.0, &filter).unwrap();
    assert_eq!(records.len(), 1);
    let record = &records[0];
    assert_eq!(record.source_run_id, "run");
    assert_eq!(record.quality_score, 0.8);
    assert_eq!(record.prompt, "");
    assert_eq!(record.completion, "");
    assert_eq!(record.tools_used, ["read", "write"]);
    let relaxed = HarvestFilter {
        only_passed_gates: false,
        ..Default::default()
    };
    assert_eq!(harvest_from_dir(&dir.0, &relaxed).unwrap().len(), 3);
    let zero = HarvestFilter {
        max_records: 0,
        ..Default::default()
    };
    assert!(harvest_from_dir(&dir.0, &zero).unwrap().is_empty());
}
