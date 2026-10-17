//! Pattern priority and match-comparison snapshots.
use crate::oracle::GrepOracle;
#[test]
fn pattern_priority_snapshot() {
    let queries = [
        "find all async functions",
        "list all public functions",
        "find all structs",
        "list all enums",
        "find all traits",
        "implementations",
        "error handling",
        "unwrap calls",
        "expect calls",
        "imports",
        "test functions",
        "async tests",
        "todo comments",
        "comments",
        "macro calls",
        "println",
        "string literals",
        "grep for 'Result<'",
        "occurrences of foo in main.rs",
        "what does this do",
    ];
    let patterns: Vec<_> = queries
        .iter()
        .map(|query| GrepOracle::infer_pattern(query))
        .collect();
    println!("PATTERNS={}", serde_json::to_string(&patterns).unwrap());
    super::comparisons::check();
}
