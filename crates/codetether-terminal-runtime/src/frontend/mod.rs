//! Frontend type and callback selection for guarded terminal drawing.
mod context;
/// Declare frontend drawing with pre-frame crash-context recording.
///
/// Size checks, panic recovery and redraw resynchronization remain in the shared
/// terminal implementation. The caller supplies concrete application/view types.
/// # Examples
/// See the frontend safe-draw module for concrete callback selection.
#[macro_export]
macro_rules! declare_frontend_safe_draw {
    ($app:path, $session:path, $terminal:ty; $record:path, $render:path $(,)?) => {
        use $app as App;
        use $session as SessionView;
        $crate::__frontend_draw_context!($record);
        /// Draw one TUI frame, recovering from transient failures.
        ///
        /// The function reads the terminal size, rejects unsafe dimensions, and
        /// delegates to the panic-guarded renderer. Size-query errors, draw errors,
        /// and render-closure panics are all logged and swallowed so the event loop
        /// continues after clearing and retrying the interrupted frame once.
        ///
        /// # Returns
        ///
        /// `Ok(true)` only when the initial draw or recovery draw rendered a frame;
        /// `Ok(false)` when a frame was skipped or recovery failed.
        ///
        /// # Side Effects
        ///
        /// Writes a frame to stdout on success. Emits `tracing` warnings/errors on
        /// any recoverable failure.
        pub fn draw_ui(
            terminal: &mut $terminal,
            app: &mut App,
            session: &SessionView,
        ) -> anyhow::Result<bool> {
            safe_draw_context::record(app, session);
            $crate::draw_ui(terminal, |frame| $render(frame, app, session))
        }
    };
}
