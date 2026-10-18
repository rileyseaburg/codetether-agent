//! Frontend session availability and deferred resume.
/// Implementation detail of [crate::declare_frontend_goal_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_goal_resume_host {
    ($resume:path) => {
        mod resume_host {
            use super::*;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub slot: &'a mut SessionSlot,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
                pub bridge: &'a Option<TuiWorkerBridge>,
                pub runtime: &'a TuiSessionHandle,
            }
            impl $crate::input::Resume for Native<'_> {
                fn processing(&self) -> bool {
                    self.app.state.processing
                }
                fn available(&self) -> bool {
                    self.slot.borrow().is_some()
                }
                fn providers(&self) -> bool {
                    self.registry.is_some()
                }
                fn session_id(&self) -> String {
                    self.slot.view().id().to_string()
                }
                async fn resume(&mut self) {
                    $resume(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.bridge,
                        self.runtime,
                    )
                    .await;
                }
            }
        }
    };
}
