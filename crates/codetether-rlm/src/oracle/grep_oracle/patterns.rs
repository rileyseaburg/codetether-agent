//! Ordered Rust query patterns.

pub(super) const PATTERNS: &[(&str, &str)] = &[
    // Async functions
    (r"(?i)find\s+all\s+async\s+functions?", r"\basync\s+fn\b"),
    (r"(?i)list\s+async\s+functions?", r"\basync\s+fn\b"),
    (r"(?i)async\s+functions?", r"\basync\s+fn\b"),
    // Public functions
    (r"(?i)public\s+functions?", r"\bpub\s+fn\b"),
    (r"(?i)pub\s+functions?", r"\bpub\s+fn\b"),
    // Structs
    (r"(?i)find\s+all\s+structs?", r"\bstruct\b"),
    (r"(?i)list\s+structs?", r"\bstruct\b"),
    (r"(?i)all\s+structs?", r"\bstruct\b"),
    // Enums
    (r"(?i)find\s+all\s+enums?", r"\benum\b"),
    (r"(?i)list\s+enums?", r"\benum\b"),
    // Traits
    (r"(?i)find\s+all\s+traits?", r"\btrait\b"),
    (r"(?i)list\s+traits?", r"\btrait\b"),
    // Impls
    (r"(?i)find\s+all\s+impls?", r"\bimpl\b"),
    (r"(?i)implementations?", r"\bimpl\b"),
    // Error handling
    (r"(?i)error\s+handling", r"Result|anyhow|Error|\?"),
    (r"(?i)unwrap\s+calls?", r"\.unwrap\(\)"),
    (r"(?i)expect\s+calls?", r"\.expect\("),
    // Imports
    (r"(?i)use\s+statements?", r"\buse\b"),
    (r"(?i)imports?", r"\buse\b"),
    // Tests
    (r"(?i)test\s+functions?", r"#\[test\]"),
    (r"(?i)async\s+tests?", r"#\[tokio::test\]"),
    // Comments
    (r"(?i)todo\s+comments?", r"TODO|FIXME|XXX"),
    (r"(?i)comments?", r"//|/\*"),
    // Macros
    (r"(?i)macro\s+calls?", r"[a-zA-Z_]+!"),
    (r"(?i)println!?", r"println!"),
    // String literals
    (r"(?i)string\s+literals?", r#""[^"]*""#),
];
