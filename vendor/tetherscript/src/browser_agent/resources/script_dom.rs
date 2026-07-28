//! DOM mutation for deterministic external script resources.

use crate::browser::{Element, Node};

use super::{script_kind, script_module, script_resolve, ResourceRegistry};

pub(crate) fn inline_node(
    node: &mut Node,
    registry: &ResourceRegistry,
    base_url: &str,
    changed: &mut bool,
) -> Result<(), String> {
    let Node::Element(element) = node else {
        return Ok(());
    };
    if script_kind::external(element) {
        inline_element(element, registry, base_url)?;
        *changed = true;
        return Ok(());
    }
    for child in &mut element.children {
        inline_node(child, registry, base_url, changed)?;
    }
    Ok(())
}

fn inline_element(
    element: &mut Element,
    registry: &ResourceRegistry,
    base_url: &str,
) -> Result<(), String> {
    let reference = element.attrs.get("src").cloned().unwrap_or_default();
    let (url, source) = if script_kind::module(element) {
        script_module::source(registry, base_url, &reference)?
    } else {
        script_resolve::text(registry, base_url, &reference)
            .map(|(url, source)| (url, source.into()))
            .ok_or_else(|| script_resolve::missing(base_url, &reference))?
    };
    element.attrs.remove("src");
    element
        .attrs
        .insert("data-agent-resource-script".into(), url);
    element.children = vec![Node::Text(source)];
    Ok(())
}
