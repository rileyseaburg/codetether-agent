//! Trusted workspace identity for direct patch backends.

use serde_json::{Value, json};
use std::path::Path;

pub(super) fn bind(args: &mut Value, root: &Path) {
    let Some(map) = args.as_object_mut() else {
        return;
    };
    map.entry("__ct_session_id".to_string())
        .or_insert_with(|| json!("direct-patch"));
    map.insert(
        "__ct_parent_workspace".to_string(),
        json!(root.display().to_string()),
    );
    crate::tool::network_access::bind_trusted(args, false);
}
