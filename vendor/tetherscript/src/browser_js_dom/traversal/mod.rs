use super::*;

mod constants;
mod filter;
mod install;
mod object;
mod object_step;
mod path;
mod root_arg;
mod state;
mod state_handle;
mod state_step;
mod walk;
mod walk_children;

use constants::*;
use filter::filter_result;
use object::traversal_object;
use object_step::install_step;
use path::cmp_path;
use root_arg::root_arg;
use state::{TraversalKind, TraversalState};
use walk::collect_paths;
use walk_children::visit_children;

type TraversalObjectRef = Rc<RefCell<Option<Rc<RefCell<HashMap<String, JsValue>>>>>>;

pub(super) fn install_window(window: &mut HashMap<String, JsValue>) {
    install::window(window);
}

pub(super) fn install_node(obj: &mut HashMap<String, JsValue>, handle: &DomHandle, node: &Node) {
    install::node(obj, handle, node);
}

#[cfg(test)]
mod tests;
