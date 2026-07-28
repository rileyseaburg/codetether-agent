use super::*;

#[path = "promise/aggregate.rs"]
mod aggregate;
#[path = "promise/api.rs"]
pub(super) mod api;
#[path = "promise/catch.rs"]
mod catch;
#[path = "promise/constructor.rs"]
mod constructor;
#[path = "promise/executor.rs"]
mod executor;
#[path = "promise/finally.rs"]
mod finally;
#[path = "promise/handler.rs"]
mod handler;
#[path = "promise/object.rs"]
mod object;
#[path = "promise/reaction.rs"]
mod reaction;
#[path = "promise/scheduler.rs"]
mod scheduler;
#[path = "promise/state.rs"]
mod state;
#[path = "promise/then.rs"]
mod then;

#[cfg(test)]
#[path = "promise_tests_all_race.rs"]
mod tests_all_race;
#[cfg(test)]
#[path = "promise_tests_constructor.rs"]
mod tests_constructor;
#[cfg(test)]
#[path = "promise_tests_microtasks.rs"]
mod tests_microtasks;
#[cfg(test)]
#[path = "promise_tests_pending_aggregate.rs"]
mod tests_pending_aggregate;
#[cfg(test)]
#[path = "promise_tests_pending_finally.rs"]
mod tests_pending_finally;
#[cfg(test)]
#[path = "promise_tests_settled.rs"]
mod tests_settled;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    scheduler::install(window.get("queueMicrotask").cloned());
    window.insert("Promise".into(), constructor::value());
}

pub(in crate::browser_js) fn reset() {
    scheduler::reset();
}
