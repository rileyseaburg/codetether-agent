//! Frontend entry into the shared loop driver.
/// Declare the frontend loop entry over its concrete resources.
///
/// Resource setup runs once before the driver takes ownership of channels,
/// bridge, runtime and setup state. The first iteration has no prior draw time.
/// # Examples
/// See the frontend event-loop module for concrete resource types and callbacks.
#[macro_export]
macro_rules! declare_frontend_loop_entry {
    ($terminal:ty, $app:ty, $registry:ty, $slot:ty, $recorder:ty,
     $bridge:ty, $event:ty, $runtime:ty, $notice:ty;
     $setup:path, $driver:path $(,)?) => {
        /// Drive terminal input, session events and periodic updates until quit.
        /// # Errors
        /// Returns an error if the shared loop driver cannot render a frame.
        pub(crate) async fn run_event_loop(
            terminal: &mut $terminal,
            app: &mut $app,
            cwd: &::std::path::Path,
            registry: Option<::std::sync::Arc<$registry>>,
            slot: &mut $slot,
            bus_recorder: ::std::sync::Arc<$recorder>,
            worker_bridge: Option<$bridge>,
            event_rx: ::tokio::sync::mpsc::Receiver<$event>,
            runtime: $runtime,
            notice_rx: ::tokio::sync::mpsc::Receiver<$notice>,
        ) -> ::anyhow::Result<()> {
            use $driver as Native;
            let setup = $setup();
            $crate::driver::run(Native {
                terminal,
                app,
                cwd,
                registry,
                slot,
                bus_recorder,
                worker_bridge,
                event_rx,
                runtime,
                notice_rx,
                setup,
                last_draw: None,
            })
            .await
        }
    };
}
