//! Pre-draw frontend refresh and rendering callbacks.
/// Implementation detail of [crate::declare_frontend_driver].
#[doc(hidden)]
#[macro_export]
macro_rules! __reactor_driver_prepare {
    ($before:path, $draw:path) => {
        mod prepare {
            use super::Native;
            impl $crate::driver::Prepare for Native<'_> {
                fn before_draw(&mut self) {
                    $before(
                        self.app,
                        &self.worker_bridge,
                        &mut self.setup.worker_sync_cursor,
                    );
                }
                async fn update_mux(&mut self) {
                    self.setup
                        .mux_status
                        .update(self.app, self.slot.view())
                        .await;
                }
                fn draw(&mut self) -> ::anyhow::Result<()> {
                    $draw(
                        self.terminal,
                        self.app,
                        self.slot.view(),
                        &mut self.last_draw,
                    )
                }
            }
        }
    };
}
