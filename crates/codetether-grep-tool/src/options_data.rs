//! Validated settings borrowed from one grep request.

pub(super) struct Options<'a> {
    pub pattern: &'a str,
    pub search_path: &'a str,
    pub is_regex: bool,
    pub include: Option<&'a str>,
    pub limit: usize,
    pub timeout_secs: u64,
    pub max_scanned_files: usize,
    pub max_file_bytes: u64,
}
