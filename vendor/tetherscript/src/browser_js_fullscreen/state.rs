use super::*;

thread_local! {
    static STATE: RefCell<ApiState> = RefCell::new(ApiState::default());
}

#[derive(Default)]
struct ApiState {
    fullscreen: Option<target::Target>,
    pointer: Option<target::Target>,
}

pub(super) fn reset() {
    STATE.with(|state| *state.borrow_mut() = ApiState::default());
}

pub(super) fn snapshot() -> (Option<target::Target>, Option<target::Target>) {
    STATE.with(|state| {
        let state = state.borrow();
        (state.fullscreen.clone(), state.pointer.clone())
    })
}

pub(super) fn request_fullscreen(target: target::Target) -> bool {
    set_target(|state| &mut state.fullscreen, target)
}

pub(super) fn request_pointer_lock(target: target::Target) -> bool {
    set_target(|state| &mut state.pointer, target)
}

pub(super) fn exit_fullscreen() -> Option<target::Target> {
    STATE.with(|state| state.borrow_mut().fullscreen.take())
}

pub(super) fn exit_pointer_lock() -> Option<target::Target> {
    STATE.with(|state| state.borrow_mut().pointer.take())
}

fn set_target(
    field: fn(&mut ApiState) -> &mut Option<target::Target>,
    target: target::Target,
) -> bool {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let slot = field(&mut state);
        if slot.as_ref().is_some_and(|current| current.same(&target)) {
            return false;
        }
        *slot = Some(target);
        true
    })
}
