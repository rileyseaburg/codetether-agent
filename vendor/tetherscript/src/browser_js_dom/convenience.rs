use super::*;

macro_rules! dom_mod {
    ($path:literal, $name:ident) => {
        #[path = $path]
        mod $name;
    };
}

dom_mod!("adjacent.rs", adjacent);
dom_mod!("adjacent_position.rs", adjacent_position);
dom_mod!("attrs.rs", attrs);
dom_mod!("client_rects.rs", client_rects);
dom_mod!("element_match/mod.rs", element_match);
dom_mod!("handle_ref.rs", handle_ref);
dom_mod!("insert_at.rs", insert_at);
dom_mod!("inserted.rs", inserted);
dom_mod!("node_args.rs", node_args);
dom_mod!("node_relations/mod.rs", node_relations);
dom_mod!("path_shift.rs", path_shift);
dom_mod!("reflected_attrs.rs", reflected_attrs);
dom_mod!("replace_children.rs", replace_children);
dom_mod!("replace_at.rs", replace_at);
dom_mod!("scroll_metrics.rs", scroll_metrics);
dom_mod!("sibling.rs", sibling);
dom_mod!("style.rs", style);
dom_mod!("style_attr.rs", style_attr);
dom_mod!("style_css_text.rs", style_css_text);
dom_mod!("style_methods.rs", style_methods);
dom_mod!("style_refresh.rs", style_refresh);
dom_mod!("style_remove.rs", style_remove);
dom_mod!("style_write.rs", style_write);

use super::attr_update;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    node_relations::install(obj, handle);
    attrs::install(obj, handle);
    reflected_attrs::install(obj, handle, node);
    element_match::install(obj, handle);
    replace_children::install(obj, handle);
    sibling::install(obj, handle);
    adjacent::install(obj, handle);
    client_rects::install(obj, handle, node);
    scroll_metrics::install(obj, handle, node);
    style::install(obj, handle, node);
}

#[cfg(test)]
#[path = "tests_replace_children.rs"]
mod tests_replace_children;
