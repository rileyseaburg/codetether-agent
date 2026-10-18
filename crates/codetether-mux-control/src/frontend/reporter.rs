//! Frontend wrapper around change-driven runtime reporting.
/// Declare the frontend reporter using the containing module's snapshot view.
///
/// # Examples
/// ```text
/// codetether_mux_control::declare_frontend_reporter!(pub(crate), App, SessionView);
/// ```
#[macro_export]
macro_rules! declare_frontend_reporter {
    ($visibility:vis, $app:path, $session:path $(,)?) => {
        use $app as App;
        use $session as SessionView;
        #[derive(Default)]
        $visibility struct Reporter {
            inner: $crate::RuntimeReporter,
        }
        impl Reporter {
            $visibility async fn update(
                &mut self,
                app: &App,
                session: &SessionView,
            ) {
                self.inner.update(snapshot::view(app, session)).await;
            }
            $visibility async fn clear(self) {
                self.inner.clear().await;
            }
        }
    };
}
