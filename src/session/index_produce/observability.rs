//! Native session bus specialization of shared summary observability.
pub type SummaryObservability =
    codetether_summary_producer::SummaryObservability<crate::session::SessionBus>;
