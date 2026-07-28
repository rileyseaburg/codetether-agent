use super::*;

#[path = "events/kind.rs"]
mod kind;
#[path = "events/methods.rs"]
mod methods;
#[path = "events/object.rs"]
mod object;

#[cfg(test)]
#[path = "events/tests.rs"]
mod tests;
#[cfg(test)]
#[path = "events/tests_path.rs"]
mod tests_path;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    obj.insert(
        "createEvent".into(),
        native("document.createEvent", Some(1), create),
    );
}

fn create(args: &[JsValue]) -> Result<JsValue, String> {
    let type_name = args.first().unwrap_or(&JsValue::Undefined).display();
    kind::validate(&type_name)?;
    Ok(object::create())
}
