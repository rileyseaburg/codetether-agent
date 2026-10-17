//! Read the context and bounded line windows.
use super::*;
impl RlmRepl {
    /// Get the context
    pub fn context(&self) -> &str {
        &self.context
    }

    /// Get context as lines
    pub fn lines(&self) -> &[String] {
        &self.context_lines
    }

    /// Get first n lines
    pub fn head(&self, n: usize) -> Vec<&str> {
        self.context_lines
            .iter()
            .take(n)
            .map(|s| s.as_str())
            .collect()
    }

    /// Get last n lines
    pub fn tail(&self, n: usize) -> Vec<&str> {
        let start = self.context_lines.len().saturating_sub(n);
        self.context_lines
            .iter()
            .skip(start)
            .map(|s| s.as_str())
            .collect()
    }
}
