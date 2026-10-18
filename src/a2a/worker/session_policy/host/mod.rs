//! Native services for the shared standard worker policy.
mod execution;
mod source;
use crate::session::Session;
pub(super) struct Native<'a> {
    pub session: &'a mut Session,
}
