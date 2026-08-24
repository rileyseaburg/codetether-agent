//! Content, workspace, and network identity for plugin approval scope.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::Path;

pub(super) fn bind(args: &mut Value, workspace: &Path, network: bool, source: &str) {
    let Some(map) = args.as_object_mut() else {
        return;
    };
    map.entry("__ct_session_id".to_string())
        .or_insert_with(|| json!("direct-tetherscript"));
    map.insert(
        "__ct_parent_workspace".into(),
        json!(workspace.display().to_string()),
    );
    map.insert(
        "__ct_tetherscript_source_sha256".into(),
        json!(hex::encode(Sha256::digest(source.as_bytes()))),
    );
    crate::tool::network_access::bind_trusted(args, network);
}
