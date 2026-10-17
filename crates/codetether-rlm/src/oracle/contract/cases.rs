//! Structured and text routing fixtures.
pub(super) const CASES: &[(&str, &str, &str)] = &[
    (
        "find all async functions",
        r#"{"kind":"grep","file":"main.rs","pattern":"async fn","matches":[{"line":1,"text":"async fn one() {}"},{"line":2,"text":"async fn two() {}"}]}"#,
        "golden",
    ),
    (
        "find all async functions",
        r#"{"kind":"grep","file":"main.rs","pattern":"async fn","matches":[{"line":1,"text":"async fn one() {}"}]}"#,
        "failed",
    ),
    (
        "find all async functions",
        r#"{"kind":"grep","file":"main.rs","pattern":"[","matches":[]}"#,
        "unverified",
    ),
    ("find all async functions", "{broken", "failed"),
    ("count async functions", "Found 2 async functions", "golden"),
    ("count async functions", "Found 1 async functions", "failed"),
    (
        "struct signature",
        r#"{"kind":"ast","file":"main.rs","query":"structs","results":[{"name":"Config"}]}"#,
        "golden",
    ),
    (
        "struct signature",
        r#"{"kind":"ast","file":"main.rs","query":"structs","results":[{"name":"Missing"}]}"#,
        "unverified",
    ),
    (
        "explain",
        r#"{"kind":"semantic","file":"main.rs","answer":"explanation"}"#,
        "unverified",
    ),
    ("show signature", "a plain answer", "unverified"),
    ("explain code", "a plain answer", "unverified"),
];
