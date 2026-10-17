//! Compatibility exports for shared inbound-message readers and task setup.
pub(super) use codetether_a2a_http::task_state::{
    blocking, fail_empty, open_task, sender_of, text_of,
};
