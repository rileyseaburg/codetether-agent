use super::*;

thread_local! {
    static DOCUMENTS: RefCell<Vec<DomObject>> = const { RefCell::new(Vec::new()) };
}

pub(super) fn reset() {
    DOCUMENTS.with(|documents| documents.borrow_mut().clear());
}

pub(super) fn register(object: &DomObject) {
    DOCUMENTS.with(|documents| documents.borrow_mut().push(object.clone()));
    refresh();
}

pub(super) fn refresh() {
    let documents = DOCUMENTS.with(|documents| documents.borrow().clone());
    let (fullscreen, pointer) = state::snapshot();
    for document in documents {
        document::refresh(&document, fullscreen.as_ref(), pointer.as_ref());
    }
}
