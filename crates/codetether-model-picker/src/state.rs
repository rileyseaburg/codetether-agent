//! Owned model picker state, independent of the application shell.
/// Model catalog, filter, selection and picker lifecycle flags.
///
/// # Examples
/// ```
/// let mut picker = codetether_model_picker::ModelPickerState::default();
/// picker.set_available_models(vec!["provider/model".into()]);
/// picker.open_model_picker();
/// assert_eq!(picker.selected_model(), Some("provider/model"));
/// ```
#[derive(Debug, Default)]
pub struct ModelPickerState {
    /// Whether provider discovery is currently in flight.
    pub model_refresh_in_flight: bool,
    /// Completion stream for the current background refresh.
    pub model_refresh_rx:
        Option<tokio::sync::mpsc::UnboundedReceiver<codetether_model_discovery::ModelRefreshEvent>>,
    /// Model to reselect after a refreshed catalog arrives.
    pub model_picker_target_model: Option<String>,
    /// Available provider-qualified model identifiers.
    pub available_models: Vec<String>,
    /// Index into the currently filtered catalog.
    pub selected_model_index: usize,
    /// Whether the picker is open.
    pub model_picker_active: bool,
    /// Current whitespace-separated query.
    pub model_filter: String,
    /// Whether confirmation should choose a verifier model.
    pub model_picker_for_verifier: bool,
}
