//! Cookie deletion detection for the JS projection.

pub(crate) fn deletes(raw: &str) -> bool {
    raw.split(';').skip(1).any(deleting_attr)
}

fn deleting_attr(part: &str) -> bool {
    let part = part.trim().to_ascii_lowercase();
    part == "max-age=0" || part.starts_with("max-age=-") || part == "expires=0"
}
