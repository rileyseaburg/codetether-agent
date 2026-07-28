use super::*;

pub(super) fn new(kind: types::Kind) -> (types::Reaction, Object) {
    let state = Rc::new(RefCell::new(state::PromiseState::Pending));
    let child_queue = super::queue();
    let object = object::from_parts(state.clone(), child_queue.clone());
    (
        types::Reaction {
            kind,
            state,
            object: object.clone(),
            queue: child_queue,
        },
        object,
    )
}
