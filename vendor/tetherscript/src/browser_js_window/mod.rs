//! Window compatibility shims for the JavaScript browser host.

use super::*;

mod bootstrap;
mod controls;
mod dispatch;
mod handlers;

pub(super) fn install_event_handlers(window: &mut HashMap<String, JsValue>) {
    handlers::install(window);
}

pub(super) fn install_dispatchers(window: &JsValue) {
    dispatch::install(window);
    controls::install(window);
}

pub(super) fn bootstrap(engine: &mut JsEngine) -> Result<(), String> {
    engine.eval(bootstrap::SOURCE).map(|_| ())
}

#[cfg(test)]
#[path = "tests_dispatch_event.rs"]
mod tests_dispatch_event;
#[cfg(test)]
#[path = "tests_handlers.rs"]
mod tests_handlers;

#[cfg(test)]
#[path = "tests_controls.rs"]
mod tests_controls;

#[cfg(test)]
#[path = "tests_popup.rs"]
mod tests_popup;

#[cfg(test)]
#[path = "tests_resize.rs"]
mod tests_resize;

#[cfg(test)]
#[path = "tests_scroll.rs"]
mod tests_scroll;
