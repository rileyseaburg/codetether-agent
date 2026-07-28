use super::*;

#[path = "aggregate/all_callbacks.rs"]
mod all_callbacks;
#[path = "aggregate/all.rs"]
mod all_impl;
#[path = "aggregate/link.rs"]
mod link;
#[path = "aggregate/race.rs"]
mod race_impl;
#[path = "aggregate/target.rs"]
mod target;

pub(super) fn all(args: &[JsValue]) -> Result<JsValue, String> {
    all_impl::run(args)
}

pub(super) fn race(args: &[JsValue]) -> Result<JsValue, String> {
    race_impl::run(args)
}

fn array_arg(args: &[JsValue], name: &str) -> Result<Rc<RefCell<Vec<JsValue>>>, String> {
    match args.first().cloned().unwrap_or(JsValue::Undefined) {
        JsValue::Array(items) => Ok(items),
        other => Err(format!("{name}: expected array, got {}", other.display())),
    }
}
