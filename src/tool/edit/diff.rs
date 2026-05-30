use similar::{ChangeTag, TextDiff};

pub struct DiffPreview {
    pub output: String,
    pub added: usize,
    pub removed: usize,
}

pub fn preview(old: &str, new: &str) -> DiffPreview {
    let mut out = String::new();
    let mut added = 0;
    let mut removed = 0;
    for change in TextDiff::from_lines(old, new).iter_all_changes() {
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
        push_line(&mut out, sign, style, &change.to_string());
    }
    DiffPreview {
        output: out,
        added,
        removed,
    }
}

fn push_line(out: &mut String, sign: &str, style: &str, text: &str) {
    let line = format!("{sign}{text}");
    match style {
        "red" => out.push_str(&format!("\x1b[31m{}\x1b[0m", line.trim_end())),
        "green" => out.push_str(&format!("\x1b[32m{}\x1b[0m", line.trim_end())),
        "default" => out.push_str(line.trim_end()),
        _ => out.push_str(line.trim_end()),
    }
    out.push('\n');
}
