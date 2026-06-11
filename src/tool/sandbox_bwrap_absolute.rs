use std::path::Path;

pub(super) fn push(specs: &mut Vec<(&'static str, String)>, op: &'static str, path: &Path) {
    if path.is_absolute() {
        let value = path.display().to_string();
        if !specs.iter().any(|(_, existing)| existing == &value) {
            specs.push((op, value));
        }
    }
}
