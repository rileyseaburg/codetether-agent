//! Input validation and authoritative environment propagation.

use super::params::{Action, Params};
use anyhow::{Result, bail};
use serde_json::json;
use std::path::PathBuf;

pub(super) fn prepare(params: &mut Params) -> Result<PathBuf> {
    if params.cwd.as_os_str().is_empty() {
        bail!("cwd must name a RustyRoad project directory");
    }
    let cwd = params.cwd.canonicalize()?;
    if !cwd.is_dir() {
        bail!("cwd must be a directory");
    }
    match params.action {
        Action::ListTools => {
            if params.tool_name.is_some() || !params.arguments.is_empty() {
                bail!("list_tools does not accept tool_name or arguments");
            }
        }
        Action::CallTool => {
            let Some(name) = params.tool_name.as_deref() else {
                bail!("call_tool requires tool_name");
            };
            if !name.starts_with("rustyroad_") {
                bail!("tool_name must be an advertised rustyroad_* tool");
            }
            let environment = params.environment.as_str();
            if let Some(value) = params.arguments.get("env")
                && value.as_str() != Some(environment)
            {
                bail!("arguments.env must match the top-level environment");
            }
            params.arguments.insert("env".into(), json!(environment));
        }
    }
    Ok(cwd)
}
