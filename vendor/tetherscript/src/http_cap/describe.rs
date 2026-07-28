use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;

use super::authority::HttpAuthority;

pub(super) fn describe(authority: &HttpAuthority) -> Value {
    let mut fields = HashMap::new();
    fields.insert("origins".into(), string_list(authority.origins.clone()));
    fields.insert("methods".into(), string_list(sorted_methods(authority)));
    fields.insert(
        "path_prefix".into(),
        authority
            .path_prefix
            .clone()
            .map(|prefix| Value::Str(Rc::new(prefix)))
            .unwrap_or(Value::Nil),
    );
    fields.insert(
        "bound_header_names".into(),
        string_list(
            authority
                .bound_headers
                .iter()
                .map(|(name, _)| name.clone())
                .collect(),
        ),
    );
    Value::Map(Rc::new(RefCell::new(fields)))
}

fn sorted_methods(authority: &HttpAuthority) -> Vec<String> {
    let mut methods: Vec<String> = authority.methods.iter().cloned().collect();
    methods.sort();
    methods
}

fn string_list(values: Vec<String>) -> Value {
    Value::List(Rc::new(RefCell::new(
        values
            .into_iter()
            .map(|value| Value::Str(Rc::new(value)))
            .collect(),
    )))
}
