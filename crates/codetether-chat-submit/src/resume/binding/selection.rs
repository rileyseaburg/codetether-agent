//! Frontend selection effects for continuation and session resume.
/// Implementation detail of [crate::declare_frontend_continuation].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_continuation_selection {
    ($picker_trait:path, $picker_state:path, $load:path) => {
        mod selection {
            use super::Native;
            use $picker_state as SessionPickerState;
            use $picker_trait as PickerHost;
            impl PickerHost for Native<'_> {
                fn picker(&self) -> &SessionPickerState {
                    &self.app.state.session_picker
                }
                async fn load_selected(&mut self, session_id: &str) {
                    if let Some(session) = self.slot.borrow_mut() {
                        $load(self.app, self.cwd, session, session_id).await;
                    }
                }
                fn current_session_id(&self) -> Option<&str> {
                    self.slot.borrow().map(|session| session.id.as_str())
                }
                async fn resume(&mut self) -> bool {
                    let available = self.registry.is_some();
                    $crate::resume::automatic::run(self, available).await
                }
            }
        }
    };
}
