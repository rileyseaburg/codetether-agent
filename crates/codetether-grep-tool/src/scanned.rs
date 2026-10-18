//! Accumulated matches and stopping conditions for one grep scan.
pub(super) struct Scanned {
    pub results: Vec<String>,
    pub scanned_files: usize,
    pub skipped_oversize: usize,
    pub skipped_unreadable: usize,
    pub timed_out: bool,
    pub scan_limit_reached: bool,
}
impl Scanned {
    pub(super) fn new() -> Self {
        Self {
            results: Vec::new(),
            scanned_files: 0usize,
            skipped_oversize: 0usize,
            skipped_unreadable: 0usize,
            timed_out: false,
            scan_limit_reached: false,
        }
    }
}
