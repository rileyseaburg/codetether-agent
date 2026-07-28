use super::*;

#[path = "methods/default.rs"]
mod default;
#[path = "methods/init.rs"]
mod init;
#[path = "methods/path.rs"]
mod path;
#[path = "methods/propagation.rs"]
mod propagation;

pub(super) fn install(event: &Rc<RefCell<HashMap<String, JsValue>>>) {
    default::install(event);
    propagation::install(event);
    path::install(event);
    init::install(event);
}
