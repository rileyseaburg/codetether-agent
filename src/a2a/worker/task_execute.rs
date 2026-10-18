//! Concrete application session adapter for native claimed-task execution.
use super::WorkerTaskRuntime;
mod sessions;

pub(super) struct Factory;
pub(super) struct Native<'a>(&'a WorkerTaskRuntime);
impl codetether_native_worker::claimed::SessionFactory for Factory {
    type Sessions<'a> = Native<'a>;
    fn sessions(runtime: &WorkerTaskRuntime) -> Self::Sessions<'_> {
        Native(runtime)
    }
}
