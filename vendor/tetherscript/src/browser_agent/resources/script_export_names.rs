//! Export name discovery for module namespace objects.

pub(crate) fn collect(source: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in source.lines().map(str::trim_start) {
        if line.starts_with("export {") || line.starts_with("export{") {
            out.extend(list_names(line));
        } else if let Some(name) = declared_name(line, "export function ") {
            out.push(name);
        } else if let Some(name) = declared_name(line, "export const ") {
            out.push(name);
        } else if let Some(name) = declared_name(line, "export let ") {
            out.push(name);
        } else if let Some(name) = declared_name(line, "export var ") {
            out.push(name);
        } else if line.starts_with("export default ") {
            out.push("default".into());
        }
    }
    out
}

fn list_names(line: &str) -> Vec<String> {
    let Some((_, rest)) = line.split_once('{') else {
        return Vec::new();
    };
    let Some((names, _)) = rest.split_once('}') else {
        return Vec::new();
    };
    names.split(',').filter_map(alias).collect()
}

fn alias(part: &str) -> Option<String> {
    let value = part.trim();
    let (_, exported) = value.split_once(" as ").unwrap_or((value, value));
    (!value.is_empty()).then(|| exported.trim().into())
}

fn declared_name(line: &str, prefix: &str) -> Option<String> {
    line.strip_prefix(prefix)
        .and_then(|tail| {
            tail.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .next()
        })
        .filter(|name| !name.is_empty())
        .map(str::to_string)
}
