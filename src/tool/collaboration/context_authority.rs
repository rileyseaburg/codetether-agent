//! Exact approval and signed network authority forwarded during delegation.

use serde::Deserialize;
use serde_json::{Map, Value, json};

#[derive(Default, Deserialize)]
pub(super) struct RuntimeAuthority {
    #[serde(default)]
    approval_id: Option<String>,
    #[serde(default, rename = "__ct_effective_network_allowed")]
    network_allowed: Option<bool>,
    #[serde(default, rename = "__ct_network_authority")]
    network_authority: Option<String>,
}

impl RuntimeAuthority {
    pub(super) fn inject(&self, payload: &mut Map<String, Value>) {
        if let Some(value) = &self.approval_id {
            payload.insert("approval_id".into(), json!(value));
        }
        if let Some(value) = self.network_allowed {
            payload.insert("__ct_effective_network_allowed".into(), json!(value));
        }
        if let Some(value) = &self.network_authority {
            payload.insert("__ct_network_authority".into(), json!(value));
        }
    }
}

impl super::RuntimeContext {
    pub(super) fn network_allowed(&self) -> Option<bool> {
        let mut payload = Map::new();
        self.inject(&mut payload);
        crate::tool::network_access::trusted_value(&Value::Object(payload))
    }
}