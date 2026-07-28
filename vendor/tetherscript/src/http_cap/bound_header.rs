use std::rc::Rc;

use crate::capability::Authority;

use super::authority::HttpAuthority;

impl HttpAuthority {
    /// Attach a harness-owned header without exposing its value to scripts.
    pub fn with_bound_header(auth: Rc<dyn Authority>, name: &str, value: &str) -> Rc<dyn Authority> {
        let this = auth
            .as_any()
            .downcast_ref::<HttpAuthority>()
            .expect("with_bound_header: authority is not HttpAuthority");
        let mut bound_headers = this.bound_headers.clone();
        bound_headers.push((name.to_string(), value.to_string()));
        Self::from_parts(
            this.origins.clone(),
            this.methods.clone(),
            this.path_prefix.clone(),
            bound_headers,
        )
    }
}
