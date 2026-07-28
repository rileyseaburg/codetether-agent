//! Module script expansion for the in-tree JavaScript runtime.

use std::collections::HashSet;

use super::{
    script_arrow, script_dynamic, script_import, script_import_bindings, script_resolve, url,
    ResourceRegistry,
};

pub(crate) fn source(
    registry: &ResourceRegistry,
    base_url: &str,
    reference: &str,
) -> Result<(String, String), String> {
    let (url, source) = script_resolve::text(registry, base_url, reference)
        .ok_or_else(|| script_resolve::missing(base_url, reference))?;
    let module_url = url::resolve(base_url, &url);
    let mut seen = HashSet::new();
    Ok((
        module_url.clone(),
        expand(registry, &module_url, source, &mut seen)?,
    ))
}

pub(crate) fn expand(
    registry: &ResourceRegistry,
    current_url: &str,
    source: &str,
    seen: &mut HashSet<String>,
) -> Result<String, String> {
    if !seen.insert(current_url.into()) {
        return Ok(String::new());
    }
    let (imports, body) = script_import::split(source);
    let mut out = String::new();
    for import in imports {
        let (url, source) = script_resolve::text(registry, current_url, &import.url)
            .ok_or_else(|| script_resolve::missing(current_url, &import.url))?;
        let module_url = url::resolve(current_url, &url);
        out.push_str(&expand(registry, &module_url, source, seen)?);
        out.push_str(&script_import_bindings::source(&import.aliases));
    }
    let body = script_dynamic::rewrite(registry, current_url, &body, seen, &mut out)?;
    out.push_str(&script_arrow::rewrite(&body));
    out.push('\n');
    Ok(out)
}
