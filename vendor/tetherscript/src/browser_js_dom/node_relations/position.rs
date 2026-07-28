use std::cmp::Ordering;

use super::*;

pub(super) fn compare(base: &DomHandle, base_id: &str, value: &JsValue) -> u16 {
    let Some(other) = dom_handle_from_value(value) else {
        return disconnected(base_id, value);
    };
    if identity::same_node(base, &other) {
        return 0;
    }
    if !identity::same_tree(base, &other) || base.node().is_none() || other.node().is_none() {
        return disconnected(base_id, value);
    }
    if identity::contains(base, &other) {
        return constants::FOLLOWING | constants::CONTAINED_BY;
    }
    if identity::contains(&other, base) {
        return constants::PRECEDING | constants::CONTAINS;
    }
    match order::path(&base.path, &other.path) {
        Ordering::Less => constants::FOLLOWING,
        Ordering::Greater => constants::PRECEDING,
        Ordering::Equal => 0,
    }
}

fn disconnected(base_id: &str, value: &JsValue) -> u16 {
    let bits = constants::DISCONNECTED | constants::IMPLEMENTATION_SPECIFIC;
    match value_id(value) {
        Some(other_id) => bits | order_bit(order::id(base_id, &other_id)),
        None => bits,
    }
}

fn order_bit(order: Ordering) -> u16 {
    match order {
        Ordering::Greater => constants::PRECEDING,
        Ordering::Less | Ordering::Equal => constants::FOLLOWING,
    }
}

fn value_id(value: &JsValue) -> Option<String> {
    let JsValue::Object(obj) = value else {
        return None;
    };
    match obj.borrow().get("__domHandleId") {
        Some(JsValue::String(id)) => Some(id.clone()),
        _ => None,
    }
}
