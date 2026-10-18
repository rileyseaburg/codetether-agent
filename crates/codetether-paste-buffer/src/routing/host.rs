use super::View;
/// Native view state and character-editing services for paste routing.
/// # Implementors
/// Extend the chat input host with symbol and filter editing operations.
pub trait Host: crate::input::Host {
    /// Whether symbol search currently takes paste priority.
    fn symbol_search_active(&self) -> bool;
    /// Append a character to symbol search.
    fn symbol_char(&mut self, ch: char);
    /// Schedule a symbol refresh after editing.
    fn refresh_symbols(&mut self);
    /// Current paste-relevant view.
    fn view(&self) -> View;
    /// Whether the protocol filter is accepting text.
    fn bus_filter_active(&self) -> bool;
    /// Append a character to the protocol filter.
    fn bus_char(&mut self, ch: char);
    /// Current protocol-filter text for its status message.
    fn bus_filter(&self) -> &str;
    /// Append a character to the model picker filter.
    fn model_char(&mut self, ch: char);
    /// Append a character to the session picker filter.
    fn session_char(&mut self, ch: char);
}
