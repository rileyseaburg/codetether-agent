use codetether_tui_models::ViewMode;
/// Synchronous view selection and Enter effects.
/// # Implementors
/// Native TUI adapters supplying current view state and detail selection.
pub trait Sync {
    /// Read the active view.
    fn view(&self) -> ViewMode;
    /// Whether Enter is editing the bus filter.
    fn bus_filter_active(&self) -> bool;
    /// Open the selected file-picker item.
    fn file_picker(&mut self);
    /// Enter Ralph detail.
    fn ralph(&mut self);
    /// Finish editing the bus filter.
    fn bus_filter(&mut self);
    /// Enter bus detail.
    fn bus_detail(&mut self);
    /// Apply the selected model.
    fn model(&mut self);
    /// Activate the selected subagent.
    fn subagents(&mut self);
}
/// Session-backed or asynchronous Enter handlers.
/// # Implementors
/// Native TUI adapters preserving their existing session borrows.
pub trait Async {
    /// Select a session.
    fn sessions(&mut self) -> impl Future<Output = ()>;
    /// Route Enter from the swarm view.
    fn swarm(&mut self) -> impl Future<Output = ()>;
    /// Submit chat input.
    fn chat(&mut self) -> impl Future<Output = ()>;
    /// Toggle the selected setting.
    fn settings(&mut self) -> impl Future<Output = ()>;
}
/// Complete view-level Enter host.
/// # Implementors
/// Automatically implemented for synchronous and asynchronous bindings.
pub trait Host: Sync + Async {}
impl<T: Sync + Async> Host for T {}
