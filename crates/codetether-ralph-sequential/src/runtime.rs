//! Borrow the live native host throughout the complete sequential run.
use crate::Host;
pub(super) struct Runtime<'a, H: Host> {
    pub(super) host: &'a mut H,
}
