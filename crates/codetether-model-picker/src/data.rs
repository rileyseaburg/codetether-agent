//! Borrowed data for one model-picker render.
/// Catalog, query and selection supplied by the application.
///
/// # Examples
/// ```
/// let catalog = vec!["provider/model".into()];
/// let view = codetether_model_picker::PickerView {
///     catalog: &catalog, filter: "model", status: "ready", selected_index: 0,
///     refreshing: false, session_model: None,
/// };
/// assert_eq!(view.catalog.len(), 1);
/// ```
pub struct PickerView<'a> {
    /// Provider model identifiers in catalog order.
    pub catalog: &'a [String],
    /// Whitespace-separated case-insensitive search terms.
    pub filter: &'a str,
    /// Status displayed in the header.
    pub status: &'a str,
    /// Highlighted index into the filtered catalog.
    pub selected_index: usize,
    /// Whether model discovery is still running.
    pub refreshing: bool,
    /// Current session model used when no filtered selection exists.
    pub session_model: Option<&'a str>,
}
