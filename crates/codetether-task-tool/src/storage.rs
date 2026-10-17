//! storage for in-memory task tracking.
use super::*;

pub(super) static TASK_COUNTER: AtomicUsize = AtomicUsize::new(1);

lazy_static::lazy_static! {
    pub(super) static ref TASK_STORE: RwLock<HashMap<String, TaskInfo>> = RwLock::new(HashMap::new());
}
