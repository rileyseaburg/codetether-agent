//! Application-local tui-worker state adapters.

/// Declare the existing tui-worker methods on a local AppState.
/// # Usage
/// Pass the state type and any native type/variant bindings required by the matcher.
#[macro_export]
macro_rules! impl_app_worker_bridge {
    ($state:ty) => {
        impl $state {
            pub fn set_worker_bridge(
                &mut self,
                worker_id: ::std::string::String,
                worker_name: ::std::string::String,
            ) {
                self.worker.set_worker_bridge(worker_id, worker_name);
            }

            pub fn sync_worker_bridge_processing(&mut self, processing: bool) {
                self.worker.sync_worker_bridge_processing(processing);
            }

            pub fn register_worker_agent(&mut self, name: ::std::string::String) {
                self.worker.register_worker_agent(name);
            }

            pub fn push_recent_task(&mut self, task: ::std::string::String) {
                self.worker.push_recent_task(task);
            }

            pub fn enqueue_worker_task(&mut self, task: $crate::IncomingTask) {
                self.worker.enqueue_worker_task(task);
            }

            pub fn dequeue_worker_task(&mut self) -> Option<$crate::IncomingTask> {
                self.worker.dequeue_worker_task()
            }
        }
    };
}
