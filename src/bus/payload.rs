//! Bounded live bus payload helpers.

pub const LIVE_BUS_MAX_BYTES: usize = 16 * 1024;

pub fn bounded(input: &str, label: &str) -> String {
    let mut out = crate::util::truncate_bytes_safe(input, LIVE_BUS_MAX_BYTES).to_string();
    if input.len() > LIVE_BUS_MAX_BYTES {
        out.push_str(label);
    }
    out
}
