//! Native lookup and presentation bindings for agent focus.
/// Native agent sources and UI effects beyond keyboard navigation.
pub trait Host: crate::Host {
    /// Accept a currently selected slash completion, returning whether it was used.
    fn apply_selected_slash_suggestion(&mut self) -> bool;
    /// Assemble the current ring from native managed/tool/swarm registries.
    fn agent_names(&self) -> Vec<String>;
    /// Active child agent, or main chat when absent.
    fn active_agent(&self) -> Option<String>;
    /// Current swarm agent names in task order.
    fn swarm_names(&self) -> Vec<Option<String>>;
    /// Agent name of the selected swarm task, if any.
    fn selected_swarm_agent(&self) -> Option<String>;
    /// Replace the active child identity.
    fn set_active_agent(&mut self, next: Option<String>);
    /// Select a swarm task in both its state and list widget.
    fn select_swarm_agent(&mut self, index: usize);
    /// Open the swarm monitor.
    fn show_swarm(&mut self);
    /// Enter swarm task detail mode.
    fn enter_swarm_detail(&mut self);
    /// Invalidate cached transcript rendering after focus changes.
    fn clear_message_cache(&mut self);
    /// Scroll to the latest transcript message.
    fn scroll_to_bottom(&mut self);
}
