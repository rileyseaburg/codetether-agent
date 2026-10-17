//! Restrict delegated authority to existing origins and scopes.
use crate::Host;
use std::rc::Rc;

use tetherscript::{capability::Authority, value::Value};

use super::computer::ComputerAuthority;
use super::computer_value;

/// Attenuate a computer authority to a subset of origins and scopes.
pub fn narrow<H: Host>(
    auth: &ComputerAuthority<H>,
    params: &Value,
) -> Result<Rc<dyn Authority>, String> {
    let origins = requested_origins(auth, params)?;
    let scopes = requested_scopes(params)?;
    if scopes
        .iter()
        .all(|scope| computer_value::scopes(auth).contains(scope))
    {
        Ok(computer_value::clone_with::<H>(origins, scopes))
    } else {
        Err("computer.narrow: requested scope is not granted".into())
    }
}

fn requested_origins<H: Host>(
    auth: &ComputerAuthority<H>,
    params: &Value,
) -> Result<Vec<String>, String> {
    let origins = match super::computer_lists::requested(params, "origins")? {
        Some(origins) => origins,
        None => return Ok(computer_value::origins(auth).to_vec()),
    };
    ensure_origin_subset(auth, &origins)?;
    Ok(origins)
}

fn ensure_origin_subset<H: Host>(
    auth: &ComputerAuthority<H>,
    origins: &[String],
) -> Result<(), String> {
    if !computer_value::origins(auth).is_empty()
        && !origins
            .iter()
            .all(|origin| computer_value::origins(auth).contains(origin))
    {
        return Err("computer.narrow: requested origins are not a subset".into());
    }
    Ok(())
}

fn requested_scopes(params: &Value) -> Result<Vec<String>, String> {
    super::computer_lists::requested(params, "scopes")?
        .ok_or_else(|| "computer.narrow: expected scopes list".into())
}
