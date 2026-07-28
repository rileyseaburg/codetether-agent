use super::*;

type Init<'a> = Option<&'a JsValue>;
type Map = HashMap<String, JsValue>;

macro_rules! forward {
    ($name:ident, $module:ident::$func:ident) => {
        pub(crate) fn $name(map: &mut Map, init: Init<'_>) {
            $module::$func(map, init);
        }
    };
}

pub(crate) fn none(_: &mut Map, _: Init<'_>) {}
forward!(animation, animation::animation);
forward!(custom, misc::custom);
forward!(mouse, mouse::insert);
forward!(keyboard, keyboard::insert);
forward!(input, text::input);
forward!(submit, misc::submit);
forward!(focus, misc::focus);
forward!(composition, interactions::composition);
forward!(touch, interactions::touch);
forward!(storage, storage::insert);
forward!(clipboard, clipboard::insert);
forward!(media_query_list, media_query::insert);
forward!(device_orientation, device::orientation);
forward!(device_motion, device::motion);
forward!(pop_state, lifecycle::pop_state);
forward!(hash_change, lifecycle::hash_change);
forward!(page_transition, lifecycle::page_transition);
forward!(before_unload, lifecycle::before_unload);
forward!(progress, lifecycle::progress);
forward!(message, message_event::insert);
forward!(error, error_event::insert);
forward!(close, close_event::insert);
forward!(transition, animation::transition);
forward!(promise_rejection, promise_rejection::insert);

pub(crate) fn pointer_event(map: &mut Map, init: Init<'_>) {
    mouse::insert(map, init);
    pointer::insert(map, init);
}

pub(crate) fn wheel_event(map: &mut Map, init: Init<'_>) {
    mouse::insert(map, init);
    wheel::insert(map, init);
}

pub(crate) fn drag_event(map: &mut Map, init: Init<'_>) {
    mouse::insert(map, init);
    interactions::drag(map, init);
}
