use std::{collections::HashSet, rc::Rc};

use tetherscript::{
    capability::Authority,
    value::{Runtime, Value},
};

/// Computer capability grant parameters passed from tool input.
pub struct ComputerGrant {
    pub enabled: bool,
    pub origins: Vec<String>,
    pub scopes: Vec<String>,
}

/// Harness-side authority for the v1 `computer` capability.
#[derive(Clone, Debug)]
pub struct ComputerAuthority {
    pub(super) origins: Vec<String>,
    pub(super) scopes: HashSet<String>,
}

impl ComputerAuthority {
    pub fn new(origins: Vec<String>, scopes: Vec<String>) -> Rc<dyn Authority> {
        Rc::new(Self {
            origins,
            scopes: scopes.into_iter().collect(),
        })
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
