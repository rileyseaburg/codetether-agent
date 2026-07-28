use super::*;

#[path = "reaction/apply.rs"]
mod apply;
#[path = "reaction/child.rs"]
mod child;
#[path = "reaction/schedule.rs"]
mod schedule;
#[path = "reaction/settle.rs"]
mod settle;
#[path = "reaction/types.rs"]
mod types;

pub(super) type Queue = Rc<RefCell<Vec<types::Reaction>>>;
type Object = Rc<RefCell<HashMap<String, JsValue>>>;

pub(super) fn queue() -> Queue {
    Rc::new(RefCell::new(Vec::new()))
}

pub(super) fn push_then(queue: Queue, ok: JsValue, err: JsValue) -> JsValue {
    push(queue, types::Kind::Then { ok, err })
}

pub(super) fn push_finally(queue: Queue, callback: JsValue) -> JsValue {
    push(queue, types::Kind::Finally { callback })
}

pub(super) fn settled_then(ok: JsValue, err: JsValue, current: state::PromiseState) -> JsValue {
    settled(types::Kind::Then { ok, err }, current)
}

pub(super) fn settled_finally(callback: JsValue, current: state::PromiseState) -> JsValue {
    settled(types::Kind::Finally { callback }, current)
}

fn push(queue: Queue, kind: types::Kind) -> JsValue {
    let (reaction, object) = child::new(kind);
    queue.borrow_mut().push(reaction);
    JsValue::Object(object)
}

fn settled(kind: types::Kind, current: state::PromiseState) -> JsValue {
    let (reaction, object) = child::new(kind);
    schedule::reaction(reaction, current);
    JsValue::Object(object)
}

pub(super) fn settle(
    state: &Rc<RefCell<state::PromiseState>>,
    object: &Object,
    queue: &Queue,
    next: state::PromiseState,
) {
    settle::promise(state, object, queue, next);
}
