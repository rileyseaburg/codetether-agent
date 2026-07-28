//! Capability-backed recursive preload for `http_serve_static`.

use std::rc::Rc;

use crate::capability::Capability;
use crate::value::Runtime;

use super::fs_ops::list_dir;
use super::site::Site;
use super::site_builder::SiteBuilder;
use super::walker::walk_names;

#[cfg(test)]
#[path = "loader_tests.rs"]
mod loader_tests;

/// Load every reachable file under `root` into a route table.
pub(crate) fn load(rt: &mut dyn Runtime, fs: &Rc<Capability>, root: &str) -> Result<Site, String> {
    let mut builder = SiteBuilder::new();
    let names = list_dir(rt, fs, root).map_err(|e| {
        format!(
            "http_serve_static: root_dir `{}` must be a readable directory: {}",
            root, e
        )
    })?;
    walk_names(rt, fs, root, "", names, 0, &mut builder)?;
    builder.finish()
}
