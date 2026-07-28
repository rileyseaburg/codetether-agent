//! Default export rewriting for deterministic module scripts.

pub(crate) fn rewrite(source: &str) -> String {
    source
        .lines()
        .map(rewrite_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn rewrite_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];
    match trimmed.strip_prefix("export default ") {
        Some(rest) => format!(
            "{}let {} = {}",
            indent,
            super::script_export_binding::local("default"),
            rest
        ),
        None => line.into(),
    }
}
