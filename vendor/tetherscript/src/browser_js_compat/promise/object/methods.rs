use super::super::*;

pub(super) fn install(
    object: &mut HashMap<String, JsValue>,
    state: Rc<RefCell<state::PromiseState>>,
    reactions: reaction::Queue,
) {
    object.insert(
        "then".into(),
        then::method(state.clone(), reactions.clone()),
    );
    object.insert(
        "catch".into(),
        catch::method(state.clone(), reactions.clone()),
    );
    object.insert("finally".into(), finally::method(state, reactions));
}
