use super::*;

#[path = "events/base.rs"]
mod base;
#[path = "events/class.rs"]
mod class;
#[path = "events/data_transfer.rs"]
mod data_transfer;
#[path = "events/fields.rs"]
mod fields;
#[path = "events/methods.rs"]
mod methods;
#[path = "events/object.rs"]
mod object;

#[cfg(test)]
#[path = "events/tests_all.rs"]
mod tests_all;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    data_transfer::install(window);
    for event_class in class::all() {
        let name = event_class.name();
        window.insert(
            name.into(),
            native(name, None, move |args| {
                Ok(object::create(event_class, args))
            }),
        );
    }
}
