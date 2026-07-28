//! JavaScript snippets used by page actions.

#[allow(dead_code)]
pub(crate) fn click(path: &[usize]) -> String {
    format!("let n={}; n.click();", node(path))
}

pub(crate) fn fill(path: &[usize], value: &str) -> String {
    format!(
        "let n={}; n.focus(); n.inputText({});",
        node(path),
        quote(value)
    )
}

fn node(path: &[usize]) -> String {
    let mut script = String::from("document");
    for index in path {
        script.push_str(&format!(".childNodes[{}]", index));
    }
    script
}

fn quote(value: &str) -> String {
    let mut out = String::from("'");
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\'' => out.push_str("\\'"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('\'');
    out
}
