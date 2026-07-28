use super::*;

thread_local! {
    static WORKER_SCRIPTS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub(super) fn reset_all() {
    WORKER_SCRIPTS.with(|scripts| scripts.borrow_mut().clear());
}

pub(super) fn register(url: &str, source: &str) {
    WORKER_SCRIPTS.with(|scripts| {
        scripts.borrow_mut().insert(url.into(), source.into());
    });
}

pub(super) fn get(url: &str) -> Option<String> {
    WORKER_SCRIPTS.with(|scripts| scripts.borrow().get(url).cloned())
}
