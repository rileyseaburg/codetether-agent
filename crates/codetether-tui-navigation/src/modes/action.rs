/// Synchronous mode-specific input operation.
/// # Examples
/// ```
/// use codetether_tui_navigation::modes::Action;
/// assert!(matches!(Action::Attach, Action::Attach));
/// ```
pub enum Action {
    /// Attach the selected file.
    Attach,
    /// Delete a character from the session filter.
    FilterBackspace,
    /// Execute the bus view's g binding.
    BusG,
    /// Execute the bus view's c binding.
    BusC,
    /// Open the bus filter input.
    BusFilter,
    /// Toggle the audit filter.
    AuditFilter,
}
/// Asynchronous setting toggle requiring an available session.
/// # Examples
/// ```
/// use codetether_tui_navigation::modes::Setting;
/// assert!(matches!(Setting::AutoApply, Setting::AutoApply));
/// ```
pub enum Setting {
    /// Toggle automatic edit application.
    AutoApply,
    /// Toggle network access.
    Network,
    /// Toggle slash autocomplete.
    Autocomplete,
}
