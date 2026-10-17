//! KeyResult data and domain behavior.

mod create;
mod data;
mod metric;
mod mutation;
mod progress;
mod status;
mod validate;
pub use data::KeyResult;
pub use metric::KrMetricType;
pub use status::KeyResultStatus;
