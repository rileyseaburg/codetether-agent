//! Sequencing of one normalized provider response.
//!
//! Implement the guard, effect and tool bindings, then call [handle]. The driver
//! owns ordering and early returns; native sessions own storage and execution.
mod effects;
mod guards;
mod handle;
mod salvage;
mod tools;
pub use effects::Effects;
pub use guards::{Guards, ProviderView};
pub use handle::handle;
pub use tools::Tools;
/// Parsed provider tool-call tuple, preserving wire ID, name and JSON input.
pub type ParsedCall = (String, String, serde_json::Value);
/// Complete native dependency boundary for a response driver.
/// # Implementors
/// Types implementing all guard, effect and tool bindings receive this marker.
pub trait Host: Guards + Effects + Tools {}
impl<T: Guards + Effects + Tools> Host for T {}
