//! Traverse selected files under the original search bounds.
use super::*;
pub(super) async fn scan(
    options: &Options<'_>,
    regex: &Regex,
    include_pattern: Option<glob::Pattern>,
) -> Scanned {
    let started = Instant::now();
    let deadline = started + Duration::from_secs(options.timeout_secs);
    let mut scanned = Scanned::new();
    let mut walker = WalkBuilder::new(options.search_path);
    walker.hidden(false).git_ignore(true);
    for entry in walker.build() {
        if Instant::now() >= deadline {
            scanned.timed_out = true;
            break;
        }
        if scanned.results.len() >= options.limit {
            break;
        }
        let Some(entry) = select::entry(entry, &include_pattern) else {
            continue;
        };
        let path = entry.path();
        scanned.scanned_files += 1;
        if scanned.scanned_files > options.max_scanned_files {
            scanned.scan_limit_reached = true;
            break;
        }
        if !scan_file::scan(path, deadline, options, regex, &mut scanned).await {
            break;
        }
    }
    scanned
}
