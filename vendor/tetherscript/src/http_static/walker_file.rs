//! File insertion helper for the static preload walker.

use std::rc::Rc;

use crate::capability::Capability;
use crate::value::Runtime;

use super::fs_ops::read_file;
use super::site_builder::SiteBuilder;

/// Read one file through the capability and add it to the route builder.
pub(crate) fn add_file(
    rt: &mut dyn Runtime,
    fs: &Rc<Capability>,
    builder: &mut SiteBuilder,
    route: String,
    path: String,
    list_error: &str,
) -> Result<(), String> {
    let bytes = read_file(rt, fs, &path, list_error)?;
    builder.add_file(route, &path, bytes)
}
