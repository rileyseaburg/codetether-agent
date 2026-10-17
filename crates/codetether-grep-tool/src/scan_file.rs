//! Update scan state after a bounded read of one selected file.
use super::*;
pub(super) async fn scan(
    path: &std::path::Path,
    deadline: Instant,
    options: &Options<'_>,
    regex: &Regex,
    scanned: &mut Scanned,
) -> bool {
    let content = match read_file::read(path, deadline, options.max_file_bytes).await {
        ReadOutcome::Content(content) => content,
        ReadOutcome::Unreadable => {
            scanned.skipped_unreadable += 1;
            return true;
        }
        ReadOutcome::Oversize => {
            scanned.skipped_oversize += 1;
            return true;
        }
        ReadOutcome::TimedOut => {
            scanned.timed_out = true;
            return false;
        }
    };
    lines::scan(&content, path, regex, options.limit, &mut scanned.results);
    true
}
