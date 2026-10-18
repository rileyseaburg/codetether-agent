//! Consume frontend resources through its shutdown callback.
/// Implementation detail of [crate::declare_frontend_driver].
#[doc(hidden)]
#[macro_export]
macro_rules! __reactor_driver_finish {
    ($finish:path) => {
        mod finish {
            use super::Native;
            impl $crate::driver::Finish for Native<'_> {
                async fn finish(self) {
                    $finish(self.worker_bridge, &self.runtime, self.setup.mux_status).await;
                }
            }
        }
    };
}
