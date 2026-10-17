//! Prepared one-shot execution and its optional retained checkout.
use crate::EphemeralRun;
use codetether_child_workspace::Handoff;
pub(crate) struct Setup<R> {
    pub run: EphemeralRun<R>,
    pub handoff: Option<Handoff>,
}
