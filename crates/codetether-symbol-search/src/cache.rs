//! Host access to the cached language representative files.
use crate::SymbolSearchState;
use std::path::PathBuf;
impl SymbolSearchState {
    /// Returns cached representative files for the next language-server query.
    pub fn language_files(&self) -> &[PathBuf] {
        &self.language_files
    }
    /// Replaces cached representative files after an accepted query completes.
    pub fn set_language_files(&mut self, files: Vec<PathBuf>) {
        self.language_files = files;
    }
}
