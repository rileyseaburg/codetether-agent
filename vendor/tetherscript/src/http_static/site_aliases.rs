//! Route aliases for directory index files.

use std::collections::HashMap;

use super::cache::CachedResponse;

/// Add `/` and `/dir/` aliases for cached `index.html` files.
pub(crate) fn add_index_aliases(
    routes: &mut HashMap<String, CachedResponse>,
    route: &str,
    response: CachedResponse,
) {
    if route == "/index.html" {
        routes.insert("/".into(), response.clone());
    }
    if let Some(prefix) = route.strip_suffix("/index.html") {
        routes.insert(format!("{}/", prefix), response);
    }
}
