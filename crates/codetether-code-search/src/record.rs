//! record for workspace code search.

#[derive(Debug)]
pub(crate) struct Match {
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) matched_line: String,
    pub(crate) context: String,
}

impl std::fmt::Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.path, self.line, self.matched_line)
    }
}
