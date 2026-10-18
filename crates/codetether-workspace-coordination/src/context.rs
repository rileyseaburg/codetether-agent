//! Trusted runtime identity extraction for coordination requests.

use anyhow::{Context, Result};
use serde_json::Value;
use std::path::PathBuf;

pub(super) struct RuntimeContext {
    pub owner: String,
    pub agent: String,
    pub workspace: PathBuf,
}

impl RuntimeContext {
    pub(super) fn from_input(input: &Value) -> Result<Self> {
        Ok(Self {
            owner: field(input, "__ct_lease_owner")?.into(),
            agent: field(input, "__ct_agent_name")?.into(),
            workspace: field(input, "__ct_parent_workspace")?.into(),
        })
    }
}

fn field<'a>(input: &'a Value, name: &str) -> Result<&'a str> {
    input
        .get(name)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .with_context(|| format!("missing trusted runtime field {name}"))
}
