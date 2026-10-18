pub use codetether_codex_discovery::CodexSessionInfo;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CodexImportReport {
    pub imported: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PersistOutcome {
    Saved,
    Unchanged,
}
