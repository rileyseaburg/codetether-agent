//! Static site route-table builder with cache limits.

use std::collections::HashMap;

use super::cache::CachedResponse;
use super::content_type;
use super::site::Site;
use super::site_aliases::add_index_aliases;
use super::site_limits::record_file;

/// Incremental builder with file and byte limits.
pub(crate) struct SiteBuilder {
    routes: HashMap<String, CachedResponse>,
    files: usize,
    bytes: usize,
}

impl SiteBuilder {
    /// Create an empty static site builder.
    pub(crate) fn new() -> Self {
        Self {
            routes: HashMap::new(),
            files: 0,
            bytes: 0,
        }
    }

    /// Add one file and its index aliases to the route table.
    pub(crate) fn add_file(
        &mut self,
        route: String,
        source: &str,
        body: Vec<u8>,
    ) -> Result<(), String> {
        record_file(&mut self.files, &mut self.bytes, body.len())?;
        let response = CachedResponse::new(200, content_type::for_path(source), body);
        self.routes.insert(route.clone(), response.clone());
        add_index_aliases(&mut self.routes, &route, response);
        Ok(())
    }

    /// Finish the site with standard error responses.
    pub(crate) fn finish(self) -> Result<Site, String> {
        Ok(Site {
            routes: self.routes,
            not_found: error_response(404, b"not found\n"),
            method_not_allowed: error_response(405, b"method not allowed\n"),
            bad_request: error_response(400, b"bad request\n"),
        })
    }
}

fn error_response(status: u16, body: &[u8]) -> CachedResponse {
    CachedResponse::new(status, "text/plain; charset=utf-8", body.to_vec())
}
