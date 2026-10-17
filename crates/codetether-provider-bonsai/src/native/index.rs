//! Validated GGUF metadata and byte-range directory.
use super::tensor_info::TensorInfo;
use serde_json::Value;
use std::collections::HashMap;
pub(super) struct Index {
    pub metadata: HashMap<String, Value>,
    pub tensors: HashMap<String, TensorInfo>,
    pub data_offset: u64,
}
