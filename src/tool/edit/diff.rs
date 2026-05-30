use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;

pub struct DiffSummary {
    pub output: String,
    pub added: usize,
    pub removed: usize,
}

pub fn summarize(old: &str, new: &str) -> DiffSummary {
    let diff = TextDiff::from_lines(old, new);
    let mut output = String::new();
    let mut added = 0;
    let mut removed = 0;
    for change in diff.iter_all_changes() {
        let (sign, color) = match change.tag() {
            ChangeTag::Delete => {
                removed += 1;
                ("-", "31")
            }
            ChangeTag::Insert => {
                added += 1;
                ("+", "32")
            }
            ChangeTag::Equal => (" ", "0"),
        };
        push_line(&mut output, sign, color, change.to_string().trim_end());
    }
    DiffSummary {
        output,
        added,
        removed,
    }
}

fn push_line(output: &mut String, sign: &str, color: &str, line: &str) {
    if color == "0" {
        output.push_str(&format!("{sign}{line}\n"));
    } else {
        output.push_str(&format!("\x1b[{color}m{sign}{line}\x1b[0m\n"));
    }
}

pub fn metadata(summary: &DiffSummary) -> HashMap<String, serde_json::Value> {
    let mut metadata = HashMap::new();
    metadata.insert("requires_confirmation".into(), serde_json::json!(true));
    metadata.insert("diff".into(), serde_json::json!(summary.output.trim()));
    metadata.insert("added_lines".into(), serde_json::json!(summary.added));
    metadata.insert("removed_lines".into(), serde_json::json!(summary.removed));
    metadata
}
