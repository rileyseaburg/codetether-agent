use crate::approval::ApprovalStore;
use std::io::Write;

/// A record must be written with one `write_all`, not streamed.
///
/// Streaming through `serde_json::to_writer` issues many small writes, so two
/// processes appending at once interleave them and shred the log into text like
/// `{{""eventevent""`. Writing sequentially from one process cannot reproduce that
/// race, so this asserts the property that prevents it: every appended record
/// occupies exactly one line.
#[test]
fn each_appended_event_occupies_exactly_one_line() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    for index in 0..5 {
        store
            .create_request("bash", "execute", &format!("cmd {index}"), "why")
            .expect("request");
    }

    let log = std::fs::read_to_string(dir.path().join("approvals.jsonl")).expect("read log");
    let lines: Vec<&str> = log.lines().filter(|line| !line.trim().is_empty()).collect();
    assert_eq!(lines.len(), 5, "one line per event: {log}");
    for line in lines {
        assert!(
            serde_json::from_str::<serde_json::Value>(line).is_ok(),
            "every line must parse alone: {line}"
        );
    }
}

/// One corrupt line must not make the whole store unreadable.
///
/// Logs written before the atomic-append fix contain interleaved records. Failing
/// the entire read would leave an operator unable to see or decide any pending
/// request, which is a worse outcome than skipping the damaged line.
#[test]
fn a_corrupt_line_is_skipped_rather_than_failing_the_read() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    let first = store
        .create_request("bash", "execute", "before", "why")
        .expect("request");

    // Exactly the shape a pair of interleaved writes produces.
    let path = dir.path().join("approvals.jsonl");
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("open log");
    writeln!(file, "{{{{\"\"eventevent\"\"::\"\"requestrequest\"\"}}}}").expect("write corruption");
    drop(file);

    let second = store
        .create_request("bash", "execute", "after", "why")
        .expect("request");

    let requests = store.events().expect("events must still be readable");
    assert_eq!(requests.len(), 2, "both good records survive");

    // And the records on either side of the damage are both intact.
    let rendered = format!("{requests:?}");
    assert!(rendered.contains(&first.id), "record before the damage");
    assert!(rendered.contains(&second.id), "record after the damage");
}
