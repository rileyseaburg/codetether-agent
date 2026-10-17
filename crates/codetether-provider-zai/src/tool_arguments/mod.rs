//! Conservative normalization and recovery of historical tool arguments.
//!
//! Use normalize for outgoing tool-call history. Recovery preserves primitive
//! fields from object-like malformed input and otherwise wraps the input.
mod normalize;
mod salvage;
pub use normalize::normalize;
pub use salvage::salvage_json_object;
