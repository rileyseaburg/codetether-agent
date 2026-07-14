use std::fs;

use serde_json::Value;

use super::report;
use crate::telemetry::memory::MemorySnapshot;

#[test]
fn report_preserves_pre_reclaim_snapshot() {
    let dir = tempfile::tempdir().expect("tempdir");
    let memory = MemorySnapshot {
        rss_kb: Some(4 * 1024 * 1024),
        peak_rss_kb: Some(4 * 1024 * 1024),
        vsize_kb: Some(8 * 1024 * 1024),
        peak_vsize_kb: Some(8 * 1024 * 1024),
        threads: Some(12),
    };
    report::write(dir.path(), 3072, &memory).expect("write report");
    let path = fs::read_dir(dir.path())
        .expect("read reports")
        .next()
        .expect("report entry")
        .expect("valid report entry")
        .path();
    let report: Value =
        serde_json::from_slice(&fs::read(path).expect("read report")).expect("valid report JSON");
    assert_eq!(report["memory"]["rss_kb"], 4 * 1024 * 1024);
    assert_eq!(report["memory"]["threads"], 12);
    assert!(
        report["panic_message"]
            .as_str()
            .unwrap()
            .contains("4096 MiB")
    );
}
