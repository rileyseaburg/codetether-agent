//! Native command intake and retained session borrow adapters.
mod intake;
mod session;
use super::{App, Arc, Path, ProviderRegistry, SessionSlot};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
}
