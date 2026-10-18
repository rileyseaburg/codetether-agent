//! Frontend cancellation state and interrupt-key bindings.
/// Declare interrupt and Control-C handlers over existing frontend state.
///
/// Shared interruption policy retains its current processing/view checks and
/// idle-exit behavior. The runtime cancellation call stays unchanged.
/// # Examples
/// See the frontend event-handler module for concrete type selection.
#[macro_export]
macro_rules! declare_frontend_interrupt {
    ($app:path, $runtime:path, $mode:path, $key:path $(,)?) => {
        mod interrupt_host {
            use $app as App;
            use $mode as ViewMode;
            use $runtime as TuiSessionHandle;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub runtime: &'a TuiSessionHandle,
            }
            impl $crate::events::interrupt::Host for Native<'_> {
                fn processing(&self) -> bool {
                    self.app.state.processing
                }
                fn in_chat(&self) -> bool {
                    self.app.state.view.mode == ViewMode::Chat
                }
                fn request_cancel(&mut self) {
                    self.runtime.request_cancel_current();
                }
                fn set_status(&mut self, status: String) {
                    self.app.state.status = status;
                }
            }
        }
        mod interrupt_key {
            use $app as App;
            use $key as KeyEvent;
            use $runtime as TuiSessionHandle;
            pub(super) fn handle(
                app: &mut App,
                runtime: &TuiSessionHandle,
                key: KeyEvent,
            ) -> Option<bool> {
                $crate::events::interrupt::handle(
                    &mut super::interrupt_host::Native { app, runtime },
                    key,
                )
            }
        }
        mod ctrl_c {
            use $app as App;
            use $runtime as TuiSessionHandle;
            pub(super) fn handle_ctrl_c(app: &mut App, runtime: &TuiSessionHandle) -> bool {
                $crate::events::interrupt::handle_ctrl_c(&mut super::interrupt_host::Native {
                    app,
                    runtime,
                })
            }
        }
    };
}
