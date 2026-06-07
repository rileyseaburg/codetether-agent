use std::path::Path;

pub fn quoted(path: &Path) -> String {
    let value = path.display().to_string();
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

pub fn with_target_dir(original: &str, line: &str) -> String {
    if original.lines().any(|row| row.trim() == "[build]") {
        return insert_into_build(original, line);
    }
    let mut updated = original.trim_end().to_string();
    if !updated.is_empty() {
        updated.push_str("\n\n");
    }
    updated.push_str("[build]\n");
    updated.push_str(line);
    updated.push('\n');
    updated
}

fn insert_into_build(original: &str, line: &str) -> String {
    let mut updated = String::new();
    for row in original.lines() {
        updated.push_str(row);
        updated.push('\n');
        if row.trim() == "[build]" {
            updated.push_str(line);
            updated.push('\n');
        }
    }
    updated
}
