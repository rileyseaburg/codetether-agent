use super::*;

thread_local! {
    static CHANNELS: RefCell<Vec<Record>> = const { RefCell::new(Vec::new()) };
    static NEXT_ID: RefCell<u64> = const { RefCell::new(1) };
}

#[derive(Clone)]
struct Record {
    id: u64,
    name: String,
    origin: String,
    object: JsObject,
}

pub(super) fn reset_all() {
    CHANNELS.with(|channels| channels.borrow_mut().clear());
    NEXT_ID.with(|next| *next.borrow_mut() = 1);
}

pub(super) fn register(name: &str, origin: &str, object: JsObject) -> u64 {
    NEXT_ID.with(|next| {
        let id = *next.borrow();
        *next.borrow_mut() = id + 1;
        CHANNELS.with(|channels| channels.borrow_mut().push(record(id, name, origin, object)));
        id
    })
}

pub(super) fn targets(sender: u64, name: &str, origin: &str) -> Vec<JsObject> {
    CHANNELS.with(|channels| {
        channels
            .borrow()
            .iter()
            .filter(|record| record.id != sender && record.name == name)
            .filter(|record| record.origin == origin && !closed(&record.object))
            .map(|record| record.object.clone())
            .collect()
    })
}

fn record(id: u64, name: &str, origin: &str, object: JsObject) -> Record {
    Record {
        id,
        name: name.into(),
        origin: origin.into(),
        object,
    }
}
