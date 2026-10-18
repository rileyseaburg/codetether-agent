//! Active-turn steering with retained-input fallback and visible feedback.
//!
//! [submit] sends expanded text and cloned images to the active run. Failed
//! sends retain attachments and queue text; accepted sends clear pending state.
mod binding;
mod input;
mod run;
mod view;
pub use input::Input;
pub use run::submit;
pub use view::View;
/// Complete native steering surface.
/// # Implementors
/// Types implementing input transport and visible state updates are hosts.
pub trait Host: Input + View {}
impl<T: Input + View> Host for T {}
