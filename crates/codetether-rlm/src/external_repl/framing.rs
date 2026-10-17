//! Decode final-answer markers emitted by the child.
use super::*;

impl ExternalRepl {
    pub(super) fn extract_final(line: &str) -> Option<String> {
        if line.contains("__FINAL__") {
            let start = line.find("__FINAL__")? + 9;
            let end = line.find("__FINAL_END__")?;
            return Some(line[start..end].to_string());
        }
        None
    }
}
