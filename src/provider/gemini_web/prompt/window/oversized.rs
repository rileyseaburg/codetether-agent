//! Strict fallback when leading instructions alone exceed the byte budget.

use crate::provider::util::truncate_bytes_safe;

pub(super) fn fit(
    rendered: &[String],
    prefix: usize,
    starts: &[usize],
    limit: usize,
    marker: &str,
) -> Vec<String> {
    if prefix >= rendered.len() {
        return vec![clipped(&rendered.join("\n"), limit)];
    }
    if limit <= marker.len() + 2 {
        return vec![truncate_bytes_safe(marker, limit).to_string()];
    }
    let latest = starts
        .iter()
        .rev()
        .nth(1)
        .copied()
        .unwrap_or(rendered.len() - 1);
    if prefix == 0 {
        let room = limit - marker.len() - 1;
        return vec![marker.to_string(), clipped(&rendered[latest], room)];
    }
    let room = limit - marker.len() - 2;
    let system_room = room / 2;
    vec![
        clipped(&rendered[..prefix].join("\n"), system_room),
        marker.to_string(),
        clipped(&rendered[latest], room - system_room),
    ]
}

fn clipped(text: &str, limit: usize) -> String {
    truncate_bytes_safe(text, limit).to_string()
}
