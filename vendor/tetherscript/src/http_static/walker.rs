//! Recursive directory walker for static preload.

use std::rc::Rc;

use crate::capability::Capability;
use crate::value::Runtime;

use super::fs_ops::list_dir;
use super::route_path::{join_path, join_route};
use super::site_builder::SiteBuilder;
use super::walker_file::add_file;

const MAX_STATIC_DEPTH: usize = 64;

/// Add all files in `names` to the static route builder.
pub(crate) fn walk_names(
    rt: &mut dyn Runtime,
    fs: &Rc<Capability>,
    fs_path: &str,
    route_prefix: &str,
    names: Vec<String>,
    depth: usize,
    builder: &mut SiteBuilder,
) -> Result<(), String> {
    if depth >= MAX_STATIC_DEPTH {
        return Err(format!(
            "http_serve_static: directory depth exceeds {MAX_STATIC_DEPTH}"
        ));
    }
    for name in names {
        walk_child(rt, fs, fs_path, route_prefix, &name, depth, builder)?;
    }
    Ok(())
}

fn walk_child(
    rt: &mut dyn Runtime,
    fs: &Rc<Capability>,
    fs_path: &str,
    route_prefix: &str,
    name: &str,
    depth: usize,
    builder: &mut SiteBuilder,
) -> Result<(), String> {
    let child_fs = join_path(fs_path, name);
    let child_route = join_route(route_prefix, name);
    match list_dir(rt, fs, &child_fs) {
        Ok(names) => walk_names(rt, fs, &child_fs, &child_route, names, depth + 1, builder),
        Err(error) => add_file(rt, fs, builder, child_route, child_fs, &error),
    }
}
