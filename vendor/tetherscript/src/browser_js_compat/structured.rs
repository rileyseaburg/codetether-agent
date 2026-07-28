use super::*;

#[path = "structured/array.rs"]
mod array;
#[path = "structured/clone.rs"]
mod clone;
#[path = "structured/key.rs"]
mod key;
#[path = "structured/object.rs"]
mod object;
#[path = "structured/state.rs"]
mod state;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "structuredClone".into(),
        native("structuredClone", None, move |args| {
            clone_value(args.first().unwrap_or(&JsValue::Undefined))
        }),
    );
}

pub(super) fn clone_value(value: &JsValue) -> Result<JsValue, String> {
    let mut state = state::CloneState::default();
    clone::clone_value(value, &mut state)
}
