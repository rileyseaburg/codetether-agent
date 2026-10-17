//! Human-readable liveness suffix for command-session headings.

use super::super::Poll;

/// Renders why a poll returned, so silent polls are not indistinguishable.
pub(super) fn suffix(poll: &Poll) -> String {
    let elapsed = format!("{:.1}s elapsed", poll.elapsed.as_secs_f64());
    let produced = bytes(poll.session_bytes);
    if !poll.running {
        return format!(" [{elapsed}, {produced} total output]");
    }
    if poll.poll_bytes > 0 {
        return format!(
            " [{elapsed}, {} this poll, {produced} total]",
            bytes(poll.poll_bytes)
        );
    }
    format!(
        " [{elapsed}, no output for {:.1}s, {produced} total]",
        poll.silent_for.as_secs_f64()
    )
}

fn bytes(count: usize) -> String {
    if count < 1024 {
        return format!("{count} B");
    }
    format!("{:.1} KiB", count as f64 / 1024.0)
}

#[cfg(test)]
#[path = "status_tests.rs"]
mod tests;
