//! HWND-targeted input planning and Windows message posting.
//!
//! [dispatch] validates requests and queues messages for a specific window.
//! Plans retain logical input state; reported queue acceptance does not confirm
//! application effects. Non-Windows calls return a failure without physical
//! input fallback. Portable planner tests do not require a desktop.
mod input {
    pub use codetether_computer_contract::input::*;
}
mod shadow;
pub use shadow::dispatch;
