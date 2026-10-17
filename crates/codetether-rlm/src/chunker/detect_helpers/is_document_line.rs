//! Classify document line.

pub(crate) fn is_document_line(line: &str) -> bool {
    if line.starts_with('#') && line.chars().nth(1).is_some_and(|c| c == ' ' || c == '#') {
        return true;
    }
    if line.starts_with("**") && line.contains("**") {
        return true;
    }
    if line.starts_with("> ") {
        return true;
    }
    if line.starts_with("- ") && line.len() > 3 {
        return true;
    }
    line.len() > 80
        && !line.ends_with('{')
        && !line.ends_with(';')
        && !line.ends_with('(')
        && !line.ends_with(')')
        && !line.ends_with('=')
}
