use super::*;

mod check;
mod control_methods;
mod controls;
mod custom;
mod dispatch;
mod elements_methods;
mod elements_object;
mod elements_sync;
mod form;
mod form_methods;
mod form_props;
mod input_attr_sync;
mod input_attrs;
mod input_compat;
mod input_indeterminate;
mod input_number;
mod input_number_arg;
mod input_step;
mod listed;
mod message;
mod names;
mod object;
mod refresh;
mod select;
#[cfg(test)]
mod tests;
#[cfg(test)]
#[path = "tests_input_compat.rs"]
mod tests_input_compat;
mod types;
mod value_setter;
mod values;

pub(super) fn install(
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    handle: &DomHandle,
    node: &Node,
) {
    dispatch::install(obj, handle, node);
}

pub(crate) fn reset() {
    custom::reset();
    select::reset();
}
