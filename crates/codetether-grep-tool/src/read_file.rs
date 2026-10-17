//! read file for bounded grep searches.
use super::*;
pub(super) enum ReadOutcome {
    Content(String),
    Unreadable,
    Oversize,
    TimedOut,
}
pub(super) async fn read(
    path: &std::path::Path,
    deadline: Instant,
    max_file_bytes: u64,
) -> ReadOutcome {
    let Some(remaining) = remaining_duration(deadline) else {
        return ReadOutcome::TimedOut;
    };
    let metadata = match tokio::time::timeout(remaining, tokio::fs::metadata(path)).await {
        Ok(Ok(metadata)) => metadata,
        Ok(Err(_)) => {
            return ReadOutcome::Unreadable;
        }
        Err(_) => {
            return ReadOutcome::TimedOut;
        }
    };
    if metadata.len() > max_file_bytes {
        return ReadOutcome::Oversize;
    }

    // Read and search file
    let Some(remaining) = remaining_duration(deadline) else {
        return ReadOutcome::TimedOut;
    };
    let content = match tokio::time::timeout(remaining, tokio::fs::read_to_string(path)).await {
        Ok(Ok(content)) => content,
        Ok(Err(_)) => {
            return ReadOutcome::Unreadable;
        }
        Err(_) => {
            return ReadOutcome::TimedOut;
        }
    };

    ReadOutcome::Content(content)
}
