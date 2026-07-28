//! Route-log matching for observed network events.

use crate::browser_agent::network::NetworkLogEntry;
use crate::browser_session::NetworkEvent;

pub fn take<'a>(
    event: &NetworkEvent,
    logs: &'a [NetworkLogEntry],
    used: &mut [bool],
) -> Option<&'a NetworkLogEntry> {
    let index = logs
        .iter()
        .enumerate()
        .find(|(index, log)| !used[*index] && matches_event(event, log))
        .map(|(index, _)| index)?;
    used[index] = true;
    logs.get(index)
}

fn matches_event(event: &NetworkEvent, log: &NetworkLogEntry) -> bool {
    event.method.eq_ignore_ascii_case(&log.method) && event.url == log.url
}
