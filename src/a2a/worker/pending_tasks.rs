//! Select application sessions and forage for the crate-owned worker dispatcher.
pub(super) type Dispatcher = codetether_native_worker::lifecycle::TaskDispatcher<
    codetether_native_worker::claimed::Executor<
        super::task_execute::Factory,
        super::task_dispatch_special::Forage,
    >,
>;
