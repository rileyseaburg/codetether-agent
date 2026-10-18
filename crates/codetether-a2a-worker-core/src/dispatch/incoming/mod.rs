//! Admission, capacity reservation, and asynchronous task-handler cleanup.
//!
//! Call [spawn_task_handler] with the shared worker runtime. [Handler] supplies
//! native task execution; this module owns when it starts and releases its slot.
mod admission;
mod handler;
mod reserve;
pub use admission::spawn_task_handler;
pub use handler::Handler;
