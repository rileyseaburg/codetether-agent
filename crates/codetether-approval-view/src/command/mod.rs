//! Interactive approval command dispatch, result presentation and release.
//!
//! [run] resolves an explicit or pending request, records its decision and
//! updates live delivery/UI state. [release_all] repeats approval while each
//! request is removed, stopping on an error or lack of progress.
mod binding;
mod dispatch;
mod feedback_binding;
mod release;
mod result;
mod view;
pub use dispatch::run;
pub use release::all as release_all;
pub use view::View;
