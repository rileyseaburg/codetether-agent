//! Frontend storage selection for text-paste sidecars.
/// Declare sidecar attachment over the frontend's paste list.
/// # Examples
/// See the frontend pasted-text module for the selected state type.
#[macro_export]
macro_rules! declare_frontend_text_paste {
    ($state:path $(,)?) => {
        use $state as AppState;
        /// Attach pasted content to the application sidecar and return its placeholder.
        pub fn attach_paste(state: &mut AppState, content: String) -> String {
            $crate::attach_paste(&mut state.pending_text_pastes, content)
        }
    };
}
