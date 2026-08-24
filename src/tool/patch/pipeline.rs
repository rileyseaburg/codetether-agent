//! End-to-end patch execution pipeline.

use super::{apply, approval_flow, args, group, metadata, parser, result, success};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Value, json};
use std::path::Path;

/// Execute a parsed patch request against the supplied workspace root.
pub(super) async fn execute(root: &Path, mut params: Value) -> Result<ToolResult> {
    super::approval_scope_bind::bind(&mut params, root);
    let mode = args::mode(&params);
    let approval_required = approval_flow::required(&mode);
    let patch = match args::patch(&params) {
        Ok(patch) => patch,
        Err(error) => {
            return Ok(metadata::attach(error, &[], 0, "", approval_required));
        }
    };
    let hunks = parser::parse_patch(&patch);
    let files = group::files(&hunks);
    if hunks.is_empty() {
        return Ok(metadata::attach(
            result::parse_error(),
            &files,
            0,
            &patch,
            approval_required,
        ));
    }
    if let Some(blocked) = super::backend_policy::blocked(&params).await {
        return Ok(metadata::attach(blocked, &files, hunks.len(), &patch, true));
    }
    if let Some(response) = approval_flow::pending(&mode, root, &files, hunks.len(), &patch) {
        return Ok(response);
    }
    let prepared = apply::prepare(root, &hunks)?;
    let receipt = match approval_flow::verify(&mode, root, &files, &patch) {
        Ok(receipt) => receipt,
        Err(error) => return Ok(metadata::attach(error, &files, hunks.len(), &patch, true)),
    };
    let outcome = apply::commit(prepared, mode.dry_run)?;
    let mut response = success::build(&outcome, mode.dry_run);
    if let Some(receipt) = receipt {
        response = response.with_metadata("approval_receipt", json!(receipt));
    }
    Ok(metadata::attach(
        response,
        &files,
        hunks.len(),
        &patch,
        false,
    ))
}
