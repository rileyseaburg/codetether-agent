//! Completed workspace query awaiting application.
use codetether_symbol_search::SymbolEntry;
use std::path::PathBuf;

pub(crate) struct Completion {
    pub query: String,
    pub files: Vec<PathBuf>,
    pub result: anyhow::Result<Vec<SymbolEntry>>,
}
