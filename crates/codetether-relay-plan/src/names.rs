//! Relay profile name normalization and uniqueness.
use crate::text::truncate_with_ellipsis;

pub(crate) fn slugify_label(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_dash = false;

    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

pub(crate) fn sanitize_relay_agent_name(value: &str) -> String {
    let raw = slugify_label(value);
    let base = if raw.is_empty() {
        "auto-specialist".to_string()
    } else if raw.starts_with("auto-") {
        raw
    } else {
        format!("auto-{raw}")
    };

    truncate_with_ellipsis(&base, 48)
        .trim_end_matches("...")
        .to_string()
}

pub(crate) fn unique_relay_agent_name(base: &str, existing: &[String]) -> String {
    if !existing.iter().any(|name| name == base) {
        return base.to_string();
    }

    let mut suffix = 2usize;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !existing.iter().any(|name| name == &candidate) {
            return candidate;
        }
        suffix += 1;
    }
}
