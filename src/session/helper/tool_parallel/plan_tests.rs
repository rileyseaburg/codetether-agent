//! Tests for ordered mixed-batch scheduling.

use super::plan::{Batch, build};

fn call(id: &str, name: &str) -> (String, String, serde_json::Value) {
    (id.into(), name.into(), serde_json::json!({}))
}

#[test]
fn mutation_is_an_exclusive_barrier_between_parallel_reads() {
    let calls = vec![
        call("1", "read"),
        call("2", "grep"),
        call("3", "write"),
        call("4", "list"),
        call("5", "tree"),
    ];

    assert_eq!(
        build(&calls, true),
        vec![
            Batch::Parallel(0..2),
            Batch::Single(2),
            Batch::Parallel(3..5)
        ]
    );
}

#[test]
fn parallelism_disabled_keeps_every_call_sequential() {
    let calls = vec![call("1", "read"), call("2", "grep")];
    assert_eq!(
        build(&calls, false),
        vec![Batch::Single(0), Batch::Single(1)]
    );
}
