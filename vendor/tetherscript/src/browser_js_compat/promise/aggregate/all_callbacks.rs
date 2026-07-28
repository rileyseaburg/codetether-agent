use super::super::*;
use super::target::Target;

#[derive(Clone)]
pub(super) struct Progress(Rc<RefCell<State>>);

struct State {
    remaining: usize,
    values: Vec<JsValue>,
}

impl Progress {
    pub(super) fn new(len: usize) -> Self {
        Self(Rc::new(RefCell::new(State {
            remaining: len,
            values: vec![JsValue::Undefined; len],
        })))
    }

    pub(super) fn fill(&self, index: usize, value: JsValue, target: &Target) {
        let mut state = self.0.borrow_mut();
        state.values[index] = value;
        state.remaining = state.remaining.saturating_sub(1);
        if state.remaining == 0 {
            let values = Rc::new(RefCell::new(state.values.clone()));
            target.settle(state::PromiseState::Fulfilled(JsValue::Array(values)));
        }
    }

    pub(super) fn resolver(&self, index: usize, target: Target) -> JsValue {
        let progress = self.clone();
        native("Promise.all.resolve", Some(1), move |args| {
            let value = args.first().cloned().unwrap_or(JsValue::Undefined);
            progress.fill(index, value, &target);
            Ok(JsValue::Undefined)
        })
    }
}

pub(super) fn rejecter(target: Target) -> JsValue {
    native("Promise.all.reject", Some(1), move |args| {
        let reason = args.first().cloned().unwrap_or(JsValue::Undefined);
        target.settle(state::PromiseState::Rejected(reason));
        Ok(JsValue::Undefined)
    })
}
