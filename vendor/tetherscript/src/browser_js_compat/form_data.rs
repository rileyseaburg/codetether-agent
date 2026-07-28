use super::*;

#[path = "form_data/model.rs"]
mod model;
#[path = "form_data/object.rs"]
mod object;
#[path = "form_data/read.rs"]
mod read;
#[path = "form_data/write.rs"]
mod write;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "FormData".into(),
        native("FormData", None, move |args| {
            Ok(object::create(model::from_arg(args.first())))
        }),
    );
}
