//! Native session bus specialization of shared summary observability.
pub type SummaryObservability =
    codetether_native_proactive::summary::SummaryObservability<crate::session::SessionBus>;
