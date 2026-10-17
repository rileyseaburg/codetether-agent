//! Capability field access and script value construction.
use crate::Host;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use tetherscript::{capability::Authority, value::Value};

use super::computer::ComputerAuthority;

pub fn clone_with<H: Host>(origins: Vec<String>, scopes: Vec<String>) -> Rc<dyn Authority> {
    Rc::new(ComputerAuthority {
        host: std::marker::PhantomData::<H>,
        origins,
        scopes: scopes.into_iter().collect(),
    })
}

pub fn origins<H: Host>(auth: &ComputerAuthority<H>) -> &[String] {
    &auth.origins
}

pub fn scopes<H: Host>(auth: &ComputerAuthority<H>) -> &HashSet<String> {
    &auth.scopes
}

pub fn map(entries: Vec<(String, Value)>) -> Value {
    Value::Map(Rc::new(RefCell::new(entries.into_iter().collect())))
}

pub fn str_value(value: impl Into<String>) -> Value {
    Value::Str(Rc::new(value.into()))
}
