use super::*;

thread_local! {
    static RANGE_REGISTRY: RefCell<HashMap<String, Rc<RefCell<state::RangeState>>>> =
        RefCell::new(HashMap::new());
    static NEXT_RANGE_ID: RefCell<u64> = const { RefCell::new(1) };
    static DOCUMENT_SELECTION: RefCell<Option<state::RangeState>> = const { RefCell::new(None) };
}

pub(super) fn reset() {
    RANGE_REGISTRY.with(|registry| registry.borrow_mut().clear());
    NEXT_RANGE_ID.with(|next| *next.borrow_mut() = 1);
    DOCUMENT_SELECTION.with(|selection| *selection.borrow_mut() = None);
}

pub(super) fn register_range(state: Rc<RefCell<state::RangeState>>) -> String {
    let id = NEXT_RANGE_ID.with(|next| {
        let mut next = next.borrow_mut();
        let id = *next;
        *next = next.saturating_add(1);
        id
    });
    let key = format!("r{id}");
    RANGE_REGISTRY.with(|registry| {
        registry.borrow_mut().insert(key.clone(), state);
    });
    key
}

pub(super) fn range_from_value(value: &JsValue) -> Option<Rc<RefCell<state::RangeState>>> {
    let JsValue::Object(object) = value else {
        return None;
    };
    let Some(JsValue::String(id)) = object.borrow().get("__rangeId").cloned() else {
        return None;
    };
    RANGE_REGISTRY.with(|registry| registry.borrow().get(&id).cloned())
}

pub(super) fn selection() -> Option<state::RangeState> {
    DOCUMENT_SELECTION.with(|selection| selection.borrow().clone())
}

pub(super) fn set_selection(selection: Option<state::RangeState>) {
    DOCUMENT_SELECTION.with(|target| *target.borrow_mut() = selection);
}
