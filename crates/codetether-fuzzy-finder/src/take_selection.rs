//! Selected-path extraction when the finder is confirmed.
//! Taking a selection always closes the optional finder, including empty results.
use crate::FuzzyFindState;
use std::path::PathBuf;
/// Read the selected path and close the finder.
///
/// # Arguments
/// * `finder` — Optional finder state to close.
///
/// # Returns
/// The selected file path, if an active finder has a current result.
///
/// # Examples
/// ```
/// let mut finder = Some(codetether_fuzzy_finder::FuzzyFindState::default());
/// assert!(codetether_fuzzy_finder::take_selected_path(&mut finder).is_none());
/// assert!(finder.is_none());
/// ```
pub fn take_selected_path(finder: &mut Option<FuzzyFindState>) -> Option<PathBuf> {
    let chosen = finder
        .as_ref()
        .and_then(|f| f.current())
        .map(|e| e.path.clone());
    *finder = None;
    chosen
}
