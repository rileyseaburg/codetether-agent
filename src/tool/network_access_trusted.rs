//! Unforgeable per-session network authority carried in internal tool input.

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

const TOKEN: &str = "__ct_network_authority";
static SECRET: OnceLock<String> = OnceLock::new();

pub(crate) fn bind_trusted(args: &mut Value, allowed: bool) {
    super::bind(args, allowed);
    let Some(token) = digest(args, allowed) else { return };
    if let Some(map) = args.as_object_mut() {
        map.insert(TOKEN.into(), json!(token));
    }
}

pub(crate) fn workspace(args: &Value) -> Option<&str> {
    value(args)?;
    args.get("__ct_parent_workspace")
        .and_then(Value::as_str)
        .filter(|workspace| !workspace.is_empty())
}

pub(crate) fn value(args: &Value) -> Option<bool> {
    let allowed = args.get(super::FIELD)?.as_bool()?;
    let supplied = args.get(TOKEN)?.as_str()?;
    (digest(args, allowed).as_deref() == Some(supplied)).then_some(allowed)
}

fn digest(args: &Value, allowed: bool) -> Option<String> {
    let session = args
        .get("__ct_session_id")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())?;
    let workspace = args
        .get("__ct_parent_workspace")
        .and_then(Value::as_str)
        .unwrap_or("");
    let secret = SECRET.get_or_init(|| uuid::Uuid::new_v4().to_string());
    let mut hash = Sha256::new();
    hash.update(secret.as_bytes());
    hash.update([0]);
    hash.update(session.as_bytes());
    hash.update([0]);
    hash.update(workspace.as_bytes());
    hash.update([u8::from(allowed)]);
    Some(hex::encode(hash.finalize()))
}