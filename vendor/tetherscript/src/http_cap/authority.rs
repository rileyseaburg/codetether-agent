use std::collections::HashSet;
use std::rc::Rc;

use crate::capability::Authority;

use super::url::normalize_origin;

/// Outgoing HTTP authority scoped by origin, method, path, and bound headers.
pub struct HttpAuthority {
    pub(super) origins: Vec<String>,
    pub(super) methods: HashSet<String>,
    pub(super) path_prefix: Option<String>,
    pub(super) bound_headers: Vec<(String, String)>,
}

impl HttpAuthority {
    /// Build a root HTTP authority for the granted origins.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(origins: Vec<String>) -> Rc<dyn Authority> {
        Self::from_parts(
            origins.into_iter().map(normalize_origin).collect(),
            default_methods(),
            None,
            Vec::new(),
        )
    }

    pub(super) fn from_parts(
        origins: Vec<String>,
        methods: HashSet<String>,
        path_prefix: Option<String>,
        bound_headers: Vec<(String, String)>,
    ) -> Rc<dyn Authority> {
        Rc::new(Self {
            origins,
            methods,
            path_prefix,
            bound_headers,
        })
    }
}

fn default_methods() -> HashSet<String> {
    ["GET", "POST", "HEAD"]
        .into_iter()
        .map(str::to_string)
        .collect()
}
