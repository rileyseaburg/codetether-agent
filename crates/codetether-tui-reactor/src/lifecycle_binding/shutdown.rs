//! Frontend mux, bridge and session-runtime shutdown sequence.
/// Declare shutdown entry points with the existing cleanup order.
///
/// # Examples
/// ```text
/// codetether_tui_reactor::declare_frontend_loop_shutdown!(Bridge, Runtime, Reporter, stop_bridge);
/// ```
#[macro_export]
macro_rules! declare_frontend_loop_shutdown {
    ($bridge:path, $runtime:path, $reporter:path, $stop:path $(,)?) => {
        use $bridge as TuiWorkerBridge;
        pub(super) async fn finish(
            bridge: Option<TuiWorkerBridge>,
            runtime: &$runtime,
            mux_status: $reporter,
        ) {
            mux_status.clear().await;
            stop_bridge(bridge).await;
            runtime.shutdown().await;
        }

        /// Deregister the TUI agent from the worker bridge.
        ///
        /// Sends a `DeregisterAgent` command to the bridge if
        /// one is active.  Errors are silently ignored.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// deregister_bridge(&bridge);
        /// ```
        pub(super) async fn stop_bridge(bridge: Option<TuiWorkerBridge>) {
            $stop(bridge.map(Into::into)).await;
        }
    };
}
