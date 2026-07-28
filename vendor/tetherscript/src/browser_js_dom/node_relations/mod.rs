use super::*;

mod constants;
mod equality;
mod identity;
mod normalize;
mod normalize_edit;
mod order;
mod position;
mod position_install;

use super::handle_ref;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    constants::install(obj);
    identity::install(obj, handle);
    equality::install(obj, handle);
    normalize::install(obj, handle);
    position_install::install(obj, handle);
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_node_methods;
