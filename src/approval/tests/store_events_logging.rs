//! Regression tests for approval-store diagnostics.

use crate::approval::ApprovalStore;
use std::io::Write;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<u8>>>);

impl Write for Capture {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0
            .lock()
            .expect("capture lock")
            .extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn corrupt_line_warning_uses_the_tracing_sink() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store = ApprovalStore::open(dir.path()).expect("store");
    std::fs::write(dir.path().join("approvals.jsonl"), "not-json\n").expect("corrupt log");
    let output = Capture::default();
    let writer = output.clone();
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_ansi(false)
        .with_target(false)
        .with_writer(move || writer.clone())
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        store.events().expect("corrupt line remains non-fatal");
    });

    let bytes = output.0.lock().expect("capture lock").clone();
    let warning = String::from_utf8(bytes).expect("UTF-8 tracing output");
    assert!(warning.contains("skipped unreadable approval log lines"));
    assert!(warning.contains("skipped=1"));
}

#[test]
fn approval_event_reads_never_write_directly_to_the_terminal() {
    let source = include_str!("../store_events.rs");
    assert!(!source.contains("eprintln!"));
    assert!(!source.contains("println!"));
}
