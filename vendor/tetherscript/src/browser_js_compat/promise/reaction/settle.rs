use super::*;

pub(super) fn promise(
    state: &Rc<RefCell<state::PromiseState>>,
    object: &Object,
    queue: &Queue,
    next: state::PromiseState,
) {
    if !matches!(*state.borrow(), state::PromiseState::Pending) {
        return;
    }
    *state.borrow_mut() = next;
    object::write_state(&mut object.borrow_mut(), &state.borrow());
    deliver(queue, state.borrow().clone());
}

fn deliver(queue: &Queue, current: state::PromiseState) {
    let reactions = queue.borrow_mut().drain(..).collect::<Vec<_>>();
    for item in reactions {
        schedule::reaction(item, current.clone());
    }
}
