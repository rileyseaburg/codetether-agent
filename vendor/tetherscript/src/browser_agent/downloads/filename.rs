//! Filename inference for deterministic downloads.

pub(crate) fn suggested(attr: Option<&String>, href: &str) -> String {
    let value = attr.map_or("", String::as_str).trim();
    if !value.is_empty() {
        return value.into();
    }
    href.split(['?', '#'])
        .next()
        .and_then(|path| path.rsplit('/').find(|part| !part.is_empty()))
        .unwrap_or("download")
        .into()
}
