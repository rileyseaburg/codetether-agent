//! Colored line diff generation and added/removed counts.
use similar::{ChangeTag, TextDiff};
pub(super) struct Preview {
    pub diff_output: String,
    pub added: usize,
    pub removed: usize,
}
pub(super) fn preview(content: &str, new_content: &str) -> Preview {
    let diff = TextDiff::from_lines(content, new_content);

    let mut diff_output = String::new();
    let mut added = 0;
    let mut removed = 0;

    for change in diff.iter_all_changes() {
        let (sign, style) = match change.tag() {
            ChangeTag::Delete => {
                removed += 1;
                ("-", "red")
            }
            ChangeTag::Insert => {
                added += 1;
                ("+", "green")
            }
            ChangeTag::Equal => (" ", "default"),
        };

        let line = format!("{}{}", sign, change);
        if style == "red" {
            diff_output.push_str(&format!("\x1b[31m{}\x1b[0m", line.trim_end()));
        } else if style == "green" {
            diff_output.push_str(&format!("\x1b[32m{}\x1b[0m", line.trim_end()));
        } else {
            diff_output.push_str(line.trim_end());
        }
        diff_output.push('\n');
    }

    Preview {
        diff_output,
        added,
        removed,
    }
}
