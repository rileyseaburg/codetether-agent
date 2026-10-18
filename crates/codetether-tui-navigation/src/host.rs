//! Read-only navigation context and native effect application.
use crate::Action;
use codetether_tui_models::ViewMode;
/// Context read at each navigation decision.
///
/// # Examples
/// ```
/// let context = codetether_tui_navigation::Context {
///     view_mode: codetether_tui_models::ViewMode::Chat,
///     show_help: false, symbols_active: false, model_picker_active: false,
///     swarm_detail: false, ralph_detail: false, bus_filter: false, bus_detail: false,
///     slash_navigable: false, subagent_detail: false,
///     subagent_scroll: 0, protocol_scroll: 0,
/// };
/// assert!(!context.show_help);
/// ```
#[derive(Debug, Clone)]
pub struct Context {
    /// Current view mode.
    pub view_mode: ViewMode,
    /// Current show help.
    pub show_help: bool,
    /// Current symbols active.
    pub symbols_active: bool,
    /// Current model picker active.
    pub model_picker_active: bool,
    /// Current swarm detail.
    pub swarm_detail: bool,
    /// Current ralph detail.
    pub ralph_detail: bool,
    /// Current bus filter.
    pub bus_filter: bool,
    /// Current bus detail.
    pub bus_detail: bool,
    /// Current slash navigable.
    pub slash_navigable: bool,
    /// Current subagent detail.
    pub subagent_detail: bool,
    /// Current subagent scroll.
    pub subagent_scroll: usize,
    /// Current protocol scroll.
    pub protocol_scroll: usize,
}
/// Bind policy to an application without exposing its state structure.
///
/// # Implementors
/// Supply current context and bindings for each native navigation operation.
pub trait Host {
    /// Read the current view and navigation flags.
    fn context(&self) -> Context;
    /// Apply one UI operation.
    fn apply(&mut self, action: Action);
    /// Give the file picker first chance to consume Escape.
    fn file_picker_escape(&mut self) -> bool;
}
