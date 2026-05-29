use std::rc::Rc;

use tetherscript::{capability::Authority, value::Value};

use super::computer::{self, ComputerAuthority};

/// Attenuate a computer authority to a subset of scopes.
pub fn narrow(auth: &ComputerAuthority, params: &Value) -> Result<Rc<dyn Authority>, String> {
    let wanted = requested_scopes(params)?;
    if wanted.iter().all(|scope| computer::scopes(auth).contains(scope)) {
        Ok(computer::clone_with_scopes(auth, wanted))
    } else {
        Err("computer.narrow: requested scope is not granted".into())
    }
}

fn requested_scopes(params: &Value) -> Result<Vec<String>, String> {
    match params {
        Value::List(items) => Ok(items
            .borrow()
            .iter()
            .filter_map(|v| match v { Value::Str(s) => Some((**s).clone()), _ => None })
            .collect()),
        Value::Map(map) => match map.borrow().get("scopes") {
            Some(Value::List(items)) => Ok(items
                .borrow()
                .iter()
                .filter_map(|v| match v { Value::Str(s) => Some((**s).clone()), _ => None })
                .collect()),
            _ => Err("computer.narrow: expected scopes list".into()),
        },
        _ => Err("computer.narrow: expected map or list".into()),
    }
}
