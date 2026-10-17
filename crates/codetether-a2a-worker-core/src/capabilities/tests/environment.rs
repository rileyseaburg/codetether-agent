//! Serialized capability environment setup and restoration.

use std::sync::Mutex;
static ENV_LOCK: Mutex<()> = Mutex::new(());
fn apply(values: [Option<&str>; 3]) {
    for (name, value) in [
        "CODETETHER_WORKER_CAPABILITIES",
        "A2A_WORKER_CAPABILITIES",
        "KNATIVE_SERVICE",
    ]
    .into_iter()
    .zip(values)
    {
        unsafe {
            match value {
                Some(value) => std::env::set_var(name, value),
                None => std::env::remove_var(name),
            }
        }
    }
}
pub(super) fn with_worker_capability_env(
    codetether_caps: Option<&str>,
    a2a_caps: Option<&str>,
    knative: Option<&str>,
    test: impl FnOnce(),
) {
    let _guard = ENV_LOCK.lock().unwrap();
    let previous = [
        "CODETETHER_WORKER_CAPABILITIES",
        "A2A_WORKER_CAPABILITIES",
        "KNATIVE_SERVICE",
    ]
    .map(|key| std::env::var(key).ok());
    apply([codetether_caps, a2a_caps, knative]);
    test();
    apply(previous.each_ref().map(Option::as_deref));
}
