//! Native active-run input and visible state adapters.
mod input;
mod view;
use super::{App, TuiSessionHandle};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub runtime: &'a TuiSessionHandle,
}
