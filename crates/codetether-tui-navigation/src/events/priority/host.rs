use crossterm::event::KeyEvent;
/// Native modal handlers for the priority chain.
/// # Implementors
/// TUI adapters retaining modal and session context.
pub trait Host {
    /// Handle an interrupt, returning its quit decision if consumed.
    fn interrupt(&mut self, key: KeyEvent) -> Option<bool>;
    /// Handle an interlude key.
    fn interlude(&mut self, key: KeyEvent) -> bool;
    /// Handle goal-prompt input.
    fn goal(&mut self, key: KeyEvent) -> bool;
    /// Whether a spawn form is active.
    fn spawn_active(&self) -> bool;
    /// Handle active spawn-form input.
    fn spawn(&mut self, key: KeyEvent) -> impl Future<Output = bool>;
    /// Handle fuzzy-file search input.
    fn fuzzy(&mut self, key: KeyEvent) -> bool;
    /// Handle asynchronous editor language-server input.
    fn editor_lsp(&mut self, key: KeyEvent) -> impl Future<Output = bool>;
    /// Handle remaining editor input.
    fn editor(&mut self, key: KeyEvent) -> bool;
}
