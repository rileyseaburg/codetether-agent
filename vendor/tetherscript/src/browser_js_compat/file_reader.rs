use super::*;

#[path = "file_reader/events.rs"]
mod events;
#[path = "file_reader/object.rs"]
mod object;
#[path = "file_reader/read.rs"]
mod read;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "FileReader".into(),
        native("FileReader", Some(0), move |_| Ok(object::create())),
    );
}
