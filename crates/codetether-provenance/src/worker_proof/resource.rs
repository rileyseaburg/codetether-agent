//! Canonical request-body binding for worker task mutations.

use anyhow::Result;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn for_json<T: Serialize + ?Sized>(task_id: &str, payload: &T) -> Result<String> {
    let body = serde_json::to_vec(payload)?;
    let digest = hex::encode(Sha256::digest(body));
    Ok(format!("{task_id}:{digest}"))
}

#[cfg(test)]
#[path = "resource_tests.rs"]
mod tests;
