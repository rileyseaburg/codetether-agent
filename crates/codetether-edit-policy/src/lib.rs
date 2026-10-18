//! Edit input policies independent of native tool execution.
//!
//! [normalize_tool_call_for_execution] maps aliases to established edit shapes.
//! [detect_stub_in_tool_input] checks replacement text for known stub markers.
//! [build_pending_confirmation_apply_request] constructs an explicit confirm call.
mod confirmation;
mod detect;
mod marker;
mod normalize;
mod normalize_multi;
mod normalize_single;
mod patch;
pub use confirmation::build_pending_confirmation_apply_request;
pub use detect::detect_stub_in_tool_input;
pub use normalize::normalize_tool_call_for_execution;
#[cfg(test)]
mod tests;
