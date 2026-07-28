use super::*;

mod closest;
mod matches;

#[cfg(test)]
mod tests;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    matches::install(obj, handle);
    closest::install(obj, handle);
}

fn selector_arg(args: &[JsValue]) -> String {
    args.first().unwrap_or(&JsValue::Undefined).display()
}

fn selector_paths(handle: &DomHandle, selector: &str) -> Vec<Vec<usize>> {
    all_by_selector(&handle.root, selector)
}
