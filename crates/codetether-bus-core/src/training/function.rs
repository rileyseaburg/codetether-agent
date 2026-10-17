//! Training record data used by the serialization pipeline.
use super::*;

/// Function name + arguments inside a tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TrainingFunction {
    pub(super) name: String,
    pub(super) arguments: String,
}
