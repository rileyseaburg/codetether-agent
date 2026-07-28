use super::super::*;

#[derive(Clone)]
pub(super) struct Target {
    state: Rc<RefCell<state::PromiseState>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    queue: reaction::Queue,
}

impl Target {
    pub(super) fn new() -> (Self, JsValue) {
        let state = Rc::new(RefCell::new(state::PromiseState::Pending));
        let queue = reaction::queue();
        let object = object::from_parts(state.clone(), queue.clone());
        (
            Self {
                state,
                object: object.clone(),
                queue,
            },
            JsValue::Object(object),
        )
    }

    pub(super) fn settle(&self, next: state::PromiseState) {
        reaction::settle(&self.state, &self.object, &self.queue, next);
    }
}
