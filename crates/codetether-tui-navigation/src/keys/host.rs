use crossterm::event::{KeyCode, KeyEvent};

/// Native input effects and a reusable navigation adapter.
/// # Implementors
/// TUI bindings supply submission, modal selectors and input mutations.
pub trait State {
    /// Navigation effects borrowing this host's current application state.
    type Navigation<'a>: crate::Host
    where
        Self: 'a;
    /// Borrow the navigation adapter for one operation.
    fn navigation(&mut self) -> Self::Navigation<'_>;
    /// Whether the current view is chat.
    fn in_chat(&self) -> bool;
    /// Whether symbol search currently owns Enter.
    fn symbols_active(&self) -> bool;
    /// Activate the selected symbol.
    fn symbol_enter(&mut self);
    /// Insert a literal newline into input.
    fn newline(&mut self);
    /// Whether Enter belongs to a pasted multiline burst.
    fn paste_newline(&self) -> bool;
    /// Whether a watchdog notice currently owns Escape.
    fn watchdog_active(&self) -> bool;
    /// Dismiss the watchdog notice.
    fn dismiss_watchdog(&mut self);
}

/// Input operations requiring native session or command bindings.
/// # Implementors
/// TUI adapters supply tab routing, model selection and submission.
pub trait Input {
    /// Dispatch forward or backward Tab.
    fn tab(&mut self, key: KeyCode);
    /// Open the model picker when a session is available.
    fn model_picker(&mut self) -> impl Future<Output = ()>;
    /// Submit normal input.
    fn enter(&mut self) -> impl Future<Output = ()>;
    /// Dispatch a remaining character or mode-specific key.
    fn other(&mut self, key: KeyEvent) -> impl Future<Output = ()>;
}

/// Complete host for shared key dispatch.
/// # Implementors
/// Automatically implemented for adapters supplying both state and input.
pub trait Host: State + Input {}
impl<T: State + Input> Host for T {}
