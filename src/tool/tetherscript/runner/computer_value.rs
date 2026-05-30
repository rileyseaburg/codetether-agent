use std::{cell::RefCell, collections::HashSet, rc::Rc};

use tetherscript::{capability::Authority, value::Value};

use super::computer::ComputerAuthority;

pub fn clone_with_scopes(auth: &ComputerAuthority, scopes: Vec<String>) -> Rc<dyn Authority> {
    Rc::new(ComputerAuthority {
        origins: auth.origins.clone(),
        scopes: scopes.into_iter().collect(),
    })
}

pub fn scopes(auth: &ComputerAuthority) -> &HashSet<String> {
    &auth.scopes
}

pub fn map(entries: Vec<(String, Value)>) -> Value {
    Value::Map(Rc::new(RefCell::new(entries.into_iter().collect())))
}

pub fn str_value(value: impl Into<String>) -> Value {
    Value::Str(Rc::new(value.into()))
}
