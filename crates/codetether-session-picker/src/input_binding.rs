//! Frontend session filter input and Enter selection bindings.
/// Declare session-picker input over the frontend's state and runtime types.
///
/// Modified characters remain ignored; Enter delegates to the selected session
/// loading callback with the same context and optional provider registry.
/// # Examples
/// See the frontend session-input module for concrete type selection.
#[macro_export]
macro_rules! declare_frontend_session_input {
    ($app:path, $modifiers:path, $slot:path, $registry:path, $bridge:path, $runtime:path;
     $select:path $(,)?) => {
        use ::std::{path::Path, sync::Arc};
        use $app as App;
        use $bridge as TuiWorkerBridge;
        use $modifiers as KeyModifiers;
        use $registry as ProviderRegistry;
        use $runtime as TuiSessionHandle;
        use $slot as SessionSlot;
        /// Append a character to the session filter string.
        ///
        /// Ignores Ctrl/Alt modified keys so they can be handled
        /// by other keybind logic.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// handle_sessions_char(&mut app, modifiers, 'a');
        /// ```
        pub fn handle_sessions_char(app: &mut App, modifiers: KeyModifiers, c: char) {
            if !modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT)
            {
                app.state.session_picker.session_filter_push(c);
            }
        }

        /// Load the currently selected session on Enter.
        ///
        /// Reads the filtered session list, finds the original
        /// index, and loads the session by ID.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// handle_enter_sessions(&mut app, cwd, &mut session).await;
        /// ```
        pub(super) async fn handle_enter_sessions(
            app: &mut App,
            cwd: &Path,
            slot: &mut SessionSlot,
            registry: &Option<Arc<ProviderRegistry>>,
            worker_bridge: &Option<TuiWorkerBridge>,
            runtime: &TuiSessionHandle,
        ) {
            $select(app, cwd, slot, registry, worker_bridge, runtime).await;
        }
    };
}
