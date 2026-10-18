//! Application persistence and metadata bindings for native task preparation.
mod state;
mod task_session_load;
pub(super) use task_session_load::Native as Loader;
