//! Frontend selection for terminal redraw state and rendering.
mod host;

/// Declare the frontend redraw adapter and its guarded draw entry point.
///
/// The frontend selects its application, session view, terminal and safe draw
/// callback. Streaming cadence and dirty-state handling remain in [crate::redraw].
///
/// # Examples
/// ```text
/// codetether_tui_tick::declare_frontend_redraw!(App, SessionView, Terminal, draw_ui);
/// ```
#[macro_export]
macro_rules! declare_frontend_redraw {
    ($app:path, $session:path, $terminal:ty, $draw:path $(,)?) => {
        $crate::__frontend_redraw_host!($app, $session, $terminal, $draw);
        pub(super) fn draw_if_ready(
            terminal: &mut $terminal,
            app: &mut $app,
            session: &$session,
            last_draw: &mut Option<::std::time::Instant>,
        ) -> ::anyhow::Result<()> {
            $crate::redraw::draw_if_ready(
                &mut host::Native {
                    terminal,
                    app,
                    session,
                },
                last_draw,
            )
        }
    };
}
