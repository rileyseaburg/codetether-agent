//! Construct request-local sampling without reloading model weights.
use super::{request::GenerationRequest, runtime::Runtime};
use candle_transformers::generation::LogitsProcessor;
pub(super) fn new(runtime: &mut Runtime, request: &GenerationRequest) -> LogitsProcessor {
    let seed = 42u64.wrapping_add(runtime.requests);
    runtime.requests = runtime.requests.wrapping_add(1);
    LogitsProcessor::new(seed, Some(request.temperature), request.top_p)
}
