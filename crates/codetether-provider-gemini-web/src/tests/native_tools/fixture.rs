//! Hidden-data fixture tools; no model-provided code or command is executed.
use anyhow::{Result, ensure};
use codetether_provider_core::ToolDefinition;
use serde_json::{Value, json};

pub(super) struct Fixture {
    pub quantity: u64,
    pub code: String,
}

impl Fixture {
    pub fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self {
            quantity: (nonce % 80 + 11) as u64,
            code: format!("native-{nonce:x}"),
        }
    }
    pub fn expected(&self) -> String {
        format!("FINAL {} {}", self.code, self.quantity * 7)
    }
    pub fn execute(&self, step: usize, name: &str, arguments: &str) -> Result<Value> {
        let args: Value = serde_json::from_str(arguments)?;
        match step {
            0 => {
                ensure!(
                    name == "lookup_probe" && args == json!({"id":"probe-native"}),
                    "unexpected lookup call"
                );
                Ok(json!({"quantity":self.quantity,"verification_code":self.code}))
            }
            1 => {
                ensure!(
                    name == "multiply" && args == json!({"a":self.quantity,"b":7}),
                    "multiply did not consume the lookup result"
                );
                Ok(json!({"product":self.quantity * 7}))
            }
            _ => anyhow::bail!("unexpected extra tool call"),
        }
    }
}

pub(super) fn tools() -> Vec<ToolDefinition> {
    serde_json::from_value(json!([
        {"name":"lookup_probe","description":"Read the quantity and verification code for a test record.",
         "parameters":{"type":"object","properties":{"id":{"type":"string"}},
             "required":["id"],"additionalProperties":false}},
        {"name":"multiply","description":"Multiply two integer arguments and return product.",
         "parameters":{"type":"object","properties":{"a":{"type":"integer"},"b":{"type":"integer"}},
             "required":["a","b"],"additionalProperties":false}}
    ])).unwrap()
}

pub(super) const TASK: &str = "Look up record probe-native using lookup_probe. Then use multiply to multiply its quantity by 7. After receiving both real tool results, return exactly FINAL followed by the record's verification_code and the product, separated by spaces. The code and quantity are not supplied here; you must obtain them from the tool.";
