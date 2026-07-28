use super::*;

#[path = "reporting_observer.rs"]
mod reporting_observer;
#[path = "scheduler.rs"]
mod scheduler;
#[path = "trusted_types.rs"]
mod trusted_types;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    base64::install(window);
    typed_array::install(window);
    text::install(window);
    structured::install(window);
    crypto::install(window);
    dom_exception::install(window);
    dom_constructors::install(window);
    events::install(window);
    blob::install(window);
    clipboard_item::install(window);
    form_data::install(window);
    file_reader::install(window);
    file_picker::install(window);
    notification::install(window);
    promise::install(window);
    reporting_observer::install(window);
    scheduler::install(window);
    url_pattern::install(window);
    trusted_types::install(window);
}
