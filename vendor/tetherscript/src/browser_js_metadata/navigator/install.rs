use super::*;

pub(super) fn install(navigator: &JsValue, route_handler: SharedBrowserJsRouteHandler) {
    let JsValue::Object(navigator_object) = navigator else {
        return;
    };
    let shared = navigator_object.clone();
    let mut navigator = navigator_object.borrow_mut();
    identity::install(&mut navigator);
    battery::install(&mut navigator);
    capabilities::install(&mut navigator);
    extras::install(&mut navigator);
    connection::install(&mut navigator);
    scheduling::install(&mut navigator);
    user_agent_data::install(&mut navigator);
    storage::install(&mut navigator);
    locks::install(&mut navigator);
    wake_lock::install(&mut navigator);
    media_capabilities::install(&mut navigator);
    media_session::install(&mut navigator);
    permissions::install(&mut navigator);
    share::install(&mut navigator, shared.clone());
    vibration::install(&mut navigator, shared);
    super::super::beacon::install(&mut navigator, route_handler);
}
