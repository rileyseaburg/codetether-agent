//! Runtime WebGL metadata store.

use super::{webgl_state::WebGlState, *};

thread_local! { static STATES: RefCell<HashMap<String, WebGlState>> = RefCell::new(HashMap::new()); }

pub(super) fn reset_all() {
    STATES.with(|states| states.borrow_mut().clear());
}

pub(super) fn ensure(handle: &DomHandle, version: u8) {
    let key = handle.event_key();
    let (width, height) = super::dimensions::dimensions(handle);
    let state = STATES.with(|states| {
        let mut states = states.borrow_mut();
        let refresh = states.get(&key).is_none_or(|state| {
            state.version != version || state.width != width || state.height != height
        });
        if refresh {
            states.insert(key.clone(), WebGlState::new(version, width, height));
        }
        states.get(&key).cloned().expect("webgl state exists")
    });
    super::webgl_attrs::sync_attrs(handle, &state);
}

pub(super) fn mutate(handle: &DomHandle, version: u8, f: impl FnOnce(&mut WebGlState)) {
    ensure(handle, version);
    let key = handle.event_key();
    let state = STATES.with(|states| {
        let mut states = states.borrow_mut();
        let state = states.get_mut(&key).expect("webgl state exists");
        f(state);
        state.clone()
    });
    super::webgl_attrs::sync_attrs(handle, &state);
}

pub(super) fn with_state<T>(
    handle: &DomHandle,
    version: u8,
    f: impl FnOnce(&WebGlState) -> T,
) -> T {
    ensure(handle, version);
    let key = handle.event_key();
    STATES.with(|states| f(states.borrow().get(&key).expect("webgl state exists")))
}
