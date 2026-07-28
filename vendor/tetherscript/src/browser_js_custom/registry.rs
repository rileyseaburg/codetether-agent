use super::*;

thread_local! {
    static DEFINITIONS: RefCell<HashMap<String, CustomDefinition>> = RefCell::new(HashMap::new());
    static WAITERS: RefCell<HashMap<String, Vec<JsValue>>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub(super) struct CustomDefinition {
    pub(super) value: JsValue,
    pub(super) observed: Vec<String>,
}

pub(super) fn reset() {
    DEFINITIONS.with(|items| items.borrow_mut().clear());
    WAITERS.with(|items| items.borrow_mut().clear());
}

pub(super) fn define(name: String, value: JsValue) -> Result<CustomDefinition, String> {
    if !util::valid_name(&name) {
        return Err(format!("customElements.define: invalid name {}", name));
    }
    let definition = CustomDefinition {
        observed: util::observed_attributes(&value),
        value,
    };
    DEFINITIONS.with(|items| {
        let mut items = items.borrow_mut();
        if items.contains_key(&name) {
            return Err(format!("customElements.define: {} already defined", name));
        }
        items.insert(name, definition.clone());
        Ok(definition)
    })
}

pub(super) fn get(name: &str) -> Option<CustomDefinition> {
    DEFINITIONS.with(|items| items.borrow().get(name).cloned())
}

pub(super) fn take_waiters(name: &str) -> Vec<JsValue> {
    WAITERS.with(|items| items.borrow_mut().remove(name).unwrap_or_default())
}

pub(super) fn wait(name: String, callback: JsValue) {
    WAITERS.with(|items| items.borrow_mut().entry(name).or_default().push(callback));
}
