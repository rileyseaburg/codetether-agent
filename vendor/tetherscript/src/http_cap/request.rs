use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::{ResultValue, Value};

use super::authority::HttpAuthority;

impl HttpAuthority {
    pub(super) fn do_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&str>,
    ) -> Result<Value, String> {
        self.check_scope(method, url)?;
        let args = vec![
            Value::Str(Rc::new(method.into())),
            Value::Str(Rc::new(url.into())),
            body.map(|value| Value::Str(Rc::new(value.into())))
                .unwrap_or(Value::Nil),
            Value::Map(Rc::new(RefCell::new(bound_headers(self)))),
        ];
        match crate::http::request(&args) {
            Value::Result(result) => match result.as_ref() {
                ResultValue::Ok(value) => Ok(value.clone()),
                ResultValue::Err(error) => {
                    Err(format!("http.{}: {}", method.to_ascii_lowercase(), error))
                }
            },
            other => Err(format!(
                "http.{}: internal client returned {}",
                method.to_ascii_lowercase(),
                other.type_name()
            )),
        }
    }
}

fn bound_headers(authority: &HttpAuthority) -> HashMap<String, Value> {
    authority
        .bound_headers
        .iter()
        .map(|(name, value)| (name.clone(), Value::Str(Rc::new(value.clone()))))
        .collect()
}
