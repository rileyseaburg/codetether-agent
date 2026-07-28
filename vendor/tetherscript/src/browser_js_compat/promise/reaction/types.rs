use super::super::*;

#[derive(Clone)]
pub(crate) enum Kind {
    Then { ok: JsValue, err: JsValue },
    Finally { callback: JsValue },
}

#[derive(Clone)]
pub(crate) struct Reaction {
    pub kind: Kind,
    pub state: Rc<RefCell<state::PromiseState>>,
    pub object: Rc<RefCell<HashMap<String, JsValue>>>,
    pub queue: super::Queue,
}
