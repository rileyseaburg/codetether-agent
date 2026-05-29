use std::{cell::RefCell, collections::HashSet, rc::Rc};

use tetherscript::{capability::Authority, value::{Runtime, Value}};

/// Computer capability grant parameters passed from tool input.
pub struct ComputerGrant {
    pub endpoint: Option<String>,
    pub origins: Vec<String>,
    pub scopes: Vec<String>,
}

/// Harness-side authority for the v1 `computer` capability.
#[derive(Clone, Debug)]
pub struct ComputerAuthority {
    endpoint: String,
    origins: Vec<String>,
    scopes: HashSet<String>,
}

impl ComputerAuthority {
    pub fn new(endpoint: &str, origins: Vec<String>, scopes: Vec<String>) -> Rc<dyn Authority> {
        Rc::new(Self { endpoint: endpoint.trim_end_matches('/').into(), origins, scopes: scopes.into_iter().collect() })
    }

    pub fn all_scopes() -> Vec<String> {
        super::computer_scopes::all()
    }
}

impl Authority for ComputerAuthority {
    fn narrow(&self, params: &Value) -> Result<Rc<dyn Authority>, String> {
        super::computer_narrow::narrow(self, params)
    }

    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        super::computer_invoke::invoke(self, method, args)
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
}

pub(super) fn clone_with_scopes(auth: &ComputerAuthority, scopes: Vec<String>) -> Rc<dyn Authority> {
    Rc::new(ComputerAuthority { endpoint: auth.endpoint.clone(), origins: auth.origins.clone(), scopes: scopes.into_iter().collect() })
}

pub(super) fn endpoint(auth: &ComputerAuthority) -> &str { &auth.endpoint }
pub(super) fn scopes(auth: &ComputerAuthority) -> &HashSet<String> { &auth.scopes }
pub(super) fn map(entries: Vec<(String, Value)>) -> Value { Value::Map(Rc::new(RefCell::new(entries.into_iter().collect()))) }
pub(super) fn str_value(value: impl Into<String>) -> Value { Value::Str(Rc::new(value.into())) }
