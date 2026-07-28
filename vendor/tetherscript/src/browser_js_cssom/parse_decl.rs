use super::model::Declaration;

pub(super) fn parse(source: &str) -> Vec<Declaration> {
    source
        .split(';')
        .filter_map(|part| {
            let (name, value) = part.split_once(':')?;
            let name = name.trim().to_ascii_lowercase();
            let value = value.trim().to_string();
            (!name.is_empty() && !value.is_empty()).then_some(Declaration { name, value })
        })
        .collect()
}

pub(super) fn css_text(declarations: &[Declaration]) -> String {
    declarations
        .iter()
        .map(|decl| format!("{}: {}", decl.name, decl.value))
        .collect::<Vec<_>>()
        .join("; ")
}
