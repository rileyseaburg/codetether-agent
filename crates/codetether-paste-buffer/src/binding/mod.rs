//! Frontend composer and view bindings for shared paste handling.
mod input;
mod routing;
mod sidecar;
/// Declare paste entry points with frontend state and attachment callbacks.
///
/// Image-first ingestion, line normalization and active-view routing remain
/// implemented by this crate. The caller selects concrete UI state and modes.
/// # Examples
/// See the frontend paste module for the selected types and callbacks.
#[macro_export]
macro_rules! declare_frontend_paste {
    ($app:path, $input_mode:path, $view_mode:path;
     $attach:path, $active:path, $refresh:path $(,)?) => {
        use $app as App;
        $crate::__frontend_paste_input!($app, $input_mode, $attach);
        $crate::__frontend_paste_routing!($app, $view_mode, $active, $refresh);
        /// Handle pasted text from the terminal.
        ///
        /// Normalises line endings, then dispatches to symbol search,
        /// bus filter, model filter, sessions filter, or chat input.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// handle_paste(&mut app, "pasted\ntext").await;
        /// ```
        pub async fn handle_paste(app: &mut App, text: &str) {
            $crate::routing::handle(app, text);
        }

        /// Insert pasted text into the chat input buffer.
        ///
        /// Shared by the bracketed-paste event path and the Ctrl+V clipboard
        /// path so both get identical image-data-URL extraction and sidecar
        /// summarisation behavior. Expects line endings already normalised.
        pub fn paste_into_chat(app: &mut App, normalized: &str) {
            $crate::input::paste_into_chat(app, normalized);
        }
    };
}
