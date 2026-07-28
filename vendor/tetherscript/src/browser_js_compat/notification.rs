use super::*;

#[path = "notification/args.rs"]
mod args;
#[path = "notification/events.rs"]
mod events;
#[path = "notification/object.rs"]
mod object;
#[path = "notification/static_api.rs"]
mod static_api;

pub(super) const PERMISSION: &str = "prompt";

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    let constructor = NativeFunction::new("Notification", None, object::construct)
        .with_property("permission", JsValue::String(PERMISSION.into()))
        .with_property("requestPermission", static_api::request_permission());
    window.insert("Notification".into(), JsValue::Native(Rc::new(constructor)));
}
