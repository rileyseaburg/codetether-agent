use super::constants::DEFAULT_VIEWPORT_WIDTH;

pub(super) fn matches(query: &str) -> bool {
    query.split(',').any(matches_part)
}

fn matches_part(part: &str) -> bool {
    let raw = part.trim().to_ascii_lowercase();
    let raw = raw.strip_prefix("only ").unwrap_or(&raw);
    if let Some(rest) = raw.strip_prefix("not ") {
        return !matches_part(rest);
    }
    raw.split("and").all(matches_one)
}

fn matches_one(raw: &str) -> bool {
    let item = raw
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim();
    if matches!(item, "" | "all" | "screen") {
        return true;
    }
    let Some((name, value)) = item.split_once(':') else {
        return false;
    };
    match name.trim() {
        "width" => px(value).is_some_and(|limit| DEFAULT_VIEWPORT_WIDTH == limit),
        "min-width" => px(value).is_some_and(|limit| DEFAULT_VIEWPORT_WIDTH >= limit),
        "max-width" => px(value).is_some_and(|limit| DEFAULT_VIEWPORT_WIDTH <= limit),
        "prefers-color-scheme" => value.trim() == "light",
        "prefers-reduced-motion" => value.trim() == "no-preference",
        "forced-colors" => value.trim() == "none",
        _ => false,
    }
}

fn px(value: &str) -> Option<i64> {
    value.trim().trim_end_matches("px").trim().parse().ok()
}
