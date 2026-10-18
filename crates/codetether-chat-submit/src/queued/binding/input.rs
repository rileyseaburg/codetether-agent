//! Frontend input operations for queued-prompt delivery.
/// Implementation detail of [crate::declare_frontend_queued_prompt].
#[doc(hidden)]
#[macro_export]
macro_rules! __queued_prompt_input {
    ($enter:path) => {
        mod input {
            use super::Native;
            use $crate::queued::State;
            impl $crate::queued::Input for Native<'_> {
                fn input(&mut self) -> State<'_> {
                    State {
                        input: &mut self.app.state.composer.input,
                        cursor: &mut self.app.state.composer.cursor,
                        status: &mut self.app.state.status,
                    }
                }
                async fn enter(&mut self) {
                    $enter(
                        self.app,
                        self.cwd,
                        self.slot,
                        self.registry,
                        self.worker_bridge,
                        self.runtime,
                    )
                    .await;
                }
            }
        }
    };
}
