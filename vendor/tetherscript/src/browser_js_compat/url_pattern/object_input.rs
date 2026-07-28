use super::*;

pub(super) fn parts(obj: &Rc<RefCell<HashMap<String, JsValue>>>, pattern: bool) -> model::Parts {
    let default = if pattern { "*" } else { "" };
    model::Parts {
        protocol: norm::protocol(&field(obj, "protocol", default)),
        hostname: norm::hostname(&field(obj, "hostname", default)),
        pathname: pathname(obj, default),
        search: norm::search(&field(obj, "search", default)),
        hash: norm::hash(&field(obj, "hash", default)),
    }
}

fn pathname(obj: &Rc<RefCell<HashMap<String, JsValue>>>, default: &str) -> String {
    norm::pathname(&field(obj, "pathname", default))
}

fn field(obj: &Rc<RefCell<HashMap<String, JsValue>>>, name: &str, default: &str) -> String {
    obj.borrow()
        .get(name)
        .map(JsValue::display)
        .unwrap_or_else(|| default.into())
}
