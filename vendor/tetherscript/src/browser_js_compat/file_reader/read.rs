use super::*;

#[path = "read/abort.rs"]
mod abort;
#[path = "read/driver.rs"]
mod driver;
#[path = "read/kind.rs"]
mod kind;
#[path = "read_state.rs"]
mod state;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: events::ListenerList,
) {
    driver::install(
        object,
        listeners.clone(),
        "readAsText",
        "FileReader.readAsText",
        kind::ReadKind::Text,
    );
    driver::install(
        object,
        listeners.clone(),
        "readAsDataURL",
        "FileReader.readAsDataURL",
        kind::ReadKind::DataUrl,
    );
    driver::install(
        object,
        listeners.clone(),
        "readAsArrayBuffer",
        "FileReader.readAsArrayBuffer",
        kind::ReadKind::ArrayBuffer,
    );
    abort::install(object, listeners);
}
