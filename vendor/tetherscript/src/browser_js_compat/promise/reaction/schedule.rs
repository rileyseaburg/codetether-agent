use super::*;

pub(super) fn reaction(item: types::Reaction, current: state::PromiseState) {
    scheduler::enqueue(native("PromiseReactionJob", Some(0), move |_| {
        apply::settle(item.clone(), current.clone());
        Ok(JsValue::Undefined)
    }));
}
