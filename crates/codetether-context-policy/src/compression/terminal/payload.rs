//! UTF-8-safe head/tail snippets and terminal truncation markers.

pub(super) fn shrink_string_payload(value: &mut String, cap_bytes: usize, label: &str) -> bool {
    if value.len() <= cap_bytes {
        return false;
    }
    *value = terminal_head_tail(value, cap_bytes, label);
    true
}

fn terminal_head_tail(value: &str, cap_bytes: usize, label: &str) -> String {
    let marker = format!(
        "\n\n[terminal context fallback truncated {label}; original_bytes={}]\n\n",
        value.len()
    );
    if cap_bytes <= marker.len() + 32 {
        return format!(
            "[terminal context fallback truncated {label}; original_bytes={}]",
            value.len()
        );
    }

    let available = cap_bytes - marker.len();
    let head_budget = available / 2;
    let tail_budget = available - head_budget;
    let head = codetether_util::truncate::truncate_bytes_safe(value, head_budget);
    let tail = suffix_bytes_safe(value, tail_budget);
    format!("{head}{marker}{tail}")
}

fn suffix_bytes_safe(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut start = s.len().saturating_sub(max_bytes);
    while start < s.len() && !s.is_char_boundary(start) {
        start += 1;
    }
    &s[start..]
}
