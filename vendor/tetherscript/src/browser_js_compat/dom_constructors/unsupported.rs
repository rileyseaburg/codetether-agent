use super::*;

const NAMES: [&str; 4] = ["Element", "HTMLElement", "Document", "DocumentFragment"];

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    for name in NAMES {
        window.insert(name.into(), constructor(name));
    }
}

fn constructor(name: &'static str) -> JsValue {
    native(name, None, move |_| {
        Err(format!("TypeError: {} constructor is not supported", name))
    })
}
