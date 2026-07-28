use super::*;

#[path = "attrs_each.rs"]
mod attrs_each;
#[path = "attrs_item.rs"]
mod attrs_item;
#[path = "attrs_lookup_tests.rs"]
#[cfg(test)]
mod attrs_lookup_tests;
#[path = "attrs_map.rs"]
mod attrs_map;
#[path = "attrs_mutation_tests.rs"]
#[cfg(test)]
mod attrs_mutation_tests;
#[path = "attrs_named.rs"]
mod attrs_named;
#[path = "attrs_names.rs"]
mod attrs_names;
#[path = "attrs_namespace.rs"]
mod attrs_namespace;
#[path = "attrs_namespace_args.rs"]
mod attrs_namespace_args;
#[path = "attrs_namespace_lookup.rs"]
mod attrs_namespace_lookup;
#[path = "attrs_namespace_meta.rs"]
mod attrs_namespace_meta;
#[path = "attrs_namespace_methods.rs"]
mod attrs_namespace_methods;
#[path = "attrs_namespace_tests.rs"]
#[cfg(test)]
mod attrs_namespace_tests;
#[path = "attrs_node.rs"]
mod attrs_node;
#[path = "attrs_owner_tests.rs"]
#[cfg(test)]
mod attrs_owner_tests;
#[path = "attrs_toggle.rs"]
mod attrs_toggle;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    attrs_map::install(obj, handle);
    attrs_namespace::install(obj, handle);
    attrs_names::install(obj, handle);
    attrs_toggle::install(obj, handle);
}
