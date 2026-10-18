//! Frontend state fields used by shared redraw policy.
/// Implementation detail of [crate::declare_frontend_redraw].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_redraw_host {
    ($app:path, $session:path, $terminal:ty, $draw:path) => {
        mod host {
            use ::std::time::Instant;
            use $app as App;
            use $session as SessionView;
            pub(super) struct Native<'a> {
                pub terminal: &'a mut $terminal,
                pub app: &'a mut App,
                pub session: &'a SessionView,
            }
            impl $crate::redraw::Host for Native<'_> {
                type Error = ::anyhow::Error;
                fn needs_redraw(&mut self) -> &mut bool {
                    &mut self.app.state.needs_redraw
                }
                fn processing(&self) -> bool {
                    self.app.state.processing
                }
                fn last_input(&self) -> Option<Instant> {
                    self.app.state.last_key_at
                }
                fn draw(&mut self) -> ::anyhow::Result<bool> {
                    $draw(self.terminal, self.app, self.session)
                }
            }
        }
    };
}
