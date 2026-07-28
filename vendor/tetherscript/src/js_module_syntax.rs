pub(crate) fn strip_exports(source: &str) -> String {
    strip_export_prefixes(&remove_export_lists(source))
}

fn remove_export_lists(source: &str) -> String {
    let mut body = source.to_string();
    while let Some(start) = find_export_list(&body) {
        let Some(end) = body[start..].find(';').map(|end| start + end) else {
            break;
        };
        body.replace_range(start..=end, "");
    }
    body
}

fn find_export_list(source: &str) -> Option<usize> {
    source.find("export{").or_else(|| source.find("export {"))
}

fn strip_export_prefixes(source: &str) -> String {
    source
        .lines()
        .map(strip_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent_len = line.len() - trimmed.len();
    let indent = &line[..indent_len];
    match trimmed.strip_prefix("export ") {
        Some(rest) if !rest.starts_with("default ") => format!("{indent}{rest}"),
        Some(_) => String::new(),
        _ => line.into(),
    }
}
