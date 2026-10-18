//! Aggregate the focused API state contracts for route assembly.
use crate::AppState;
use codetether_cognition_api::http::{Beliefs, Lifecycle, Personas, Proposals, Workspace};
impl<R: Lifecycle + Personas + Beliefs + Workspace + Proposals + 'static>
    codetether_server_router::HttpState for AppState<R>
{
}
