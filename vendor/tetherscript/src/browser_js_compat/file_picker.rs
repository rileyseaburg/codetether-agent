use super::*;

const METHODS: [&str; 3] = [
    "showOpenFilePicker",
    "showSaveFilePicker",
    "showDirectoryPicker",
];

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    for method in METHODS {
        window.insert(method.into(), picker(method));
    }
}

fn picker(name: &'static str) -> JsValue {
    native(name, None, move |_| Ok(promise::api::rejected(reason())))
}

fn reason() -> JsValue {
    let object = Rc::new(RefCell::new(HashMap::from([
        ("name".into(), JsValue::String("NotAllowedError".into())),
        (
            "message".into(),
            JsValue::String("file picker unsupported".into()),
        ),
    ])));
    object.borrow_mut().insert("toString".into(), to_string());
    JsValue::Object(object)
}

fn to_string() -> JsValue {
    native("FileSystemPickerError.toString", Some(0), |_| {
        Ok(JsValue::String(
            "NotAllowedError: file picker unsupported".into(),
        ))
    })
}

#[cfg(test)]
#[path = "file_picker/tests.rs"]
mod tests;
