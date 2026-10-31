//! Exact action vocabularies used by intent classification.
pub(super) const CONNECTORS: &[&str] = &[
    "and", "then", "to", "please", "also", "must", "should", "kindly",
];
pub(super) const RUN: &[&str] = &["run", "execute"];
pub(super) const VERIFYING: &[&str] = &[
    "test", "tests", "lint", "verify", "format", "clippy", "check",
];
pub(super) const READING: &[&str] = &[
    "design",
    "inspect",
    "research",
    "review",
    "analyze",
    "analyse",
    "audit",
    "plan",
    "fact",
    "summarize",
    "summarise",
    "explore",
    "document",
    "read",
    "search",
    "find",
    "investigate",
];
