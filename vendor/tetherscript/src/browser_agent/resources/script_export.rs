//! Export syntax rewriting for deterministic module scripts.

pub(crate) fn strip(source: &str) -> String {
    let (body, aliases) = remove_export_lists(source);
    let body = super::script_export_default::rewrite(&body);
    let body = body
        .lines()
        .map(|line| line.trim_start().strip_prefix("export ").unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}\n{}", body, aliases)
}

fn remove_export_lists(source: &str) -> (String, String) {
    let mut body = source.to_string();
    let mut aliases = String::new();
    while let Some(start) = find_export_list(&body) {
        let Some(end) = body[start..].find(';').map(|end| start + end) else {
            break;
        };
        let statement = body[start..=end].to_string();
        aliases.push_str(&alias_source(&statement));
        body.replace_range(start..=end, "");
    }
    (body, aliases)
}

fn find_export_list(source: &str) -> Option<usize> {
    source.find("export{").or_else(|| source.find("export {"))
}

fn alias_source(statement: &str) -> String {
    let Some((_, rest)) = statement.split_once('{') else {
        return String::new();
    };
    let Some((names, _)) = rest.split_once('}') else {
        return String::new();
    };
    names
        .split(',')
        .filter_map(alias)
        .map(|(local, exported)| {
            format!(
                "let {} = {};\n",
                super::script_export_binding::local(&exported),
                local
            )
        })
        .collect()
}

fn alias(part: &str) -> Option<(String, String)> {
    let value = part.trim();
    let (local, exported) = value.split_once(" as ").unwrap_or((value, value));
    (!value.is_empty()).then(|| (local.trim().into(), exported.trim().into()))
}
