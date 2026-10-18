//! Select application execution for the crate-owned worker dispatcher.
pub(super) type Dispatcher =
    codetether_native_worker::lifecycle::TaskDispatcher<super::task_handler::Native>;
