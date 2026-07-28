//! Native in-memory static HTTP server for `http_serve_static`.

mod args;
mod cache;
mod content_type;
mod fs;
mod fs_ops;
mod loader;
mod request;
mod request_body;
mod request_head;
mod request_header_scan;
mod request_parse;
mod route_path;
mod server;
mod site;
mod site_aliases;
mod site_builder;
mod site_limits;
mod walker;
mod walker_file;
mod worker;

use std::cell::RefCell;
use std::rc::Rc;

use crate::value::{Env, Runtime, Value};

use args::{port_arg, string_arg};

/// Serve files from a capability-scoped directory without per-request script calls.
pub(crate) fn serve(
    rt: &mut dyn Runtime,
    globals: &Rc<RefCell<Env>>,
    port: &Value,
    root: &Value,
) -> Result<Value, String> {
    let port = port_arg(port)?;
    let root = string_arg(root)?;
    let fs = fs::capability(globals)?;
    let site = loader::load(rt, &fs, &root)?;
    server::serve(port, site)?;
    Ok(Value::Nil)
}
