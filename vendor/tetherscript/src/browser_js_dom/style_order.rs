pub(super) fn normalize(source: &str) -> String {
    serialize(parse(source))
}

pub(crate) fn names(source: &str) -> Vec<String> {
    parse(source).into_iter().map(|(name, _)| name).collect()
}

pub(super) fn set(source: &str, name: &str, value: &str) -> String {
    let name = name.trim().to_ascii_lowercase();
    if name.is_empty() {
        return normalize(source);
    }
    let mut decls = parse(source);
    if value.is_empty() {
        decls.retain(|(current, _)| current != &name);
        return serialize(decls);
    }
    match decls.iter_mut().find(|(current, _)| current == &name) {
        Some((_, current)) => *current = value.to_string(),
        None => decls.push((name, value.to_string())),
    }
    serialize(decls)
}

pub(crate) fn remove(source: &str, name: &str) -> String {
    set(source, name, "")
}

fn parse(source: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for decl in source.split(';') {
        let Some((name, value)) = decl.split_once(':') else {
            continue;
        };
        let name = name.trim().to_ascii_lowercase();
        let value = value.trim().to_string();
        if name.is_empty() || value.is_empty() {
            continue;
        }
        match out.iter_mut().find(|(current, _)| current == &name) {
            Some((_, current)) => *current = value,
            None => out.push((name, value)),
        }
    }
    out
}

fn serialize(decls: Vec<(String, String)>) -> String {
    decls
        .into_iter()
        .map(|(name, value)| format!("{}: {};", name, value))
        .collect::<Vec<_>>()
        .join(" ")
}
