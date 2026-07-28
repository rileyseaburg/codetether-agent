use super::*;
use crate::system;

#[path = "crypto/subtle_unsupported.rs"]
mod unsupported;

pub(super) fn subtle_object() -> JsValue {
    let mut subtle = HashMap::new();
    subtle.insert(
        "digest".into(),
        native("crypto.subtle.digest", Some(2), move |args| {
            let algorithm = algorithm_name(args.first());
            if algorithm != "SHA-256" {
                return Err(format!("crypto.subtle.digest: unsupported {algorithm}"));
            }
            let data = args.get(1).unwrap_or(&JsValue::Undefined);
            Ok(fulfilled_promise(bytes::byte_array(system::sha256(
                &bytes::bytes_from_value(data),
            ))))
        }),
    );
    unsupported::install(&mut subtle);
    JsValue::Object(Rc::new(RefCell::new(subtle)))
}

fn algorithm_name(value: Option<&JsValue>) -> String {
    match value {
        Some(JsValue::Object(obj)) => obj
            .borrow()
            .get("name")
            .map(JsValue::display)
            .unwrap_or_default()
            .to_ascii_uppercase(),
        Some(value) => value.display().to_ascii_uppercase(),
        None => String::new(),
    }
}

fn fulfilled_promise(value: JsValue) -> JsValue {
    let mut promise = HashMap::new();
    promise.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    promise.insert("__promise_value".into(), value.clone());
    install_then_catch_simple(&mut promise, value);
    JsValue::Object(Rc::new(RefCell::new(promise)))
}
