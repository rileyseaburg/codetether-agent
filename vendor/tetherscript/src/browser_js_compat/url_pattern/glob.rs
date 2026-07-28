pub(super) fn matches(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') {
        return pattern == value;
    }
    ordered_segments_match(pattern, value) && anchored_end_matches(pattern, value)
}

fn ordered_segments_match(pattern: &str, value: &str) -> bool {
    let mut rest = value;
    let mut first = true;
    for segment in pattern.split('*').filter(|segment| !segment.is_empty()) {
        if first && !pattern.starts_with('*') {
            if !rest.starts_with(segment) {
                return false;
            }
            rest = &rest[segment.len()..];
        } else if let Some(index) = rest.find(segment) {
            rest = &rest[index + segment.len()..];
        } else {
            return false;
        }
        first = false;
    }
    true
}

fn anchored_end_matches(pattern: &str, value: &str) -> bool {
    pattern.ends_with('*')
        || pattern
            .rsplit('*')
            .find(|segment| !segment.is_empty())
            .is_none_or(|last| value.ends_with(last))
}
