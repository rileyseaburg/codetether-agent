//! Application render bindings grouped by responsibility.
mod chat;
mod diagnostics;
mod management;
mod overlays;
mod primary;
pub use chat::Chat;
pub use diagnostics::Diagnostics;
pub use management::Management;
pub use overlays::Overlays;
pub use primary::Primary;
/// Current view selection.
/// # Implementors
/// Application adapters read their view state.
pub trait State {
    /// Current terminal view.
    fn view(&self) -> codetether_tui_models::ViewMode;
}
/// Complete binding for [crate::render].
/// # Implementors
/// Any type implementing all render groups and [State].
pub trait Host: State + Chat + Primary + Diagnostics + Management + Overlays {}
impl<T: State + Chat + Primary + Diagnostics + Management + Overlays> Host for T {}
