//! External script application for page resources.

use crate::browser::Document;

use super::{inline, script_dom, ResourceRegistry};

pub(crate) fn inline_scripts(
    document: &Document,
    registry: &ResourceRegistry,
    base_url: &str,
) -> Result<Option<String>, String> {
    let mut document = document.clone();
    let mut changed = false;
    for child in &mut document.children {
        script_dom::inline_node(child, registry, base_url, &mut changed)?;
    }
    Ok(changed.then(|| inline::document_html(&document)))
}
