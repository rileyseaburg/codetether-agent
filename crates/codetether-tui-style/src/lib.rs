//! Shared terminal animation, pane styling, and welcome presentation.
//!
//! These helpers use wall-clock animation and terminal color capability,
//! without owning application state. Modules preserve the original UI APIs.
pub mod banner;
pub mod border_style;
mod elapsed;
pub mod gradient_rule;
pub mod gradient_sweep;
pub mod input_preview;
pub mod mode_accent;
pub mod spinner;
pub mod status;
pub mod thinking_pulse;
pub use codetether_theme::gradient;
