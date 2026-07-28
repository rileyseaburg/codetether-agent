use std::collections::HashMap;
use std::rc::Rc;

use crate::capability::Authority;
use crate::value::Value;

use super::authority::HttpAuthority;

pub(super) fn narrow(
    authority: &HttpAuthority,
    params: &Value,
) -> Result<Rc<dyn Authority>, String> {
    let map = expect_map(params)?;
    let mut origins = authority.origins.clone();
    let mut methods = authority.methods.clone();
    let mut path_prefix = authority.path_prefix.clone();

    super::narrow_origins::apply(map.get("origins"), &mut origins)?;
    super::narrow_methods::apply(map.get("methods"), &mut methods)?;
    super::narrow_path::apply(map.get("path_prefix"), &mut path_prefix)?;

    Ok(HttpAuthority::from_parts(
        origins,
        methods,
        path_prefix,
        authority.bound_headers.clone(),
    ))
}

fn expect_map(params: &Value) -> Result<std::cell::Ref<'_, HashMap<String, Value>>, String> {
    match params {
        Value::Map(map) => Ok(map.borrow()),
        _ => Err("http.narrow: expected a map of params".into()),
    }
}
