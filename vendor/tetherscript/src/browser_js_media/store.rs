use super::model::MediaState;
use super::*;

thread_local! {
    static MEDIA_STATES: RefCell<HashMap<String, MediaState>> = RefCell::new(HashMap::new());
}

pub(super) fn get(handle: &DomHandle) -> MediaState {
    let key = handle.event_key();
    MEDIA_STATES.with(|states| {
        states
            .borrow_mut()
            .entry(key)
            .or_insert_with(|| initial(handle))
            .clone()
    })
}

pub(super) fn update(handle: &DomHandle, change: impl FnOnce(&mut MediaState)) -> MediaState {
    let key = handle.event_key();
    MEDIA_STATES.with(|states| {
        let mut states = states.borrow_mut();
        let state = states.entry(key).or_insert_with(|| initial(handle));
        change(state);
        state.clone()
    })
}

pub(super) fn reset_all() {
    MEDIA_STATES.with(|states| states.borrow_mut().clear());
}

fn initial(handle: &DomHandle) -> MediaState {
    match handle.node() {
        Some(Node::Element(el)) => MediaState::from_attrs(&el.attrs),
        _ => MediaState::default(),
    }
}
