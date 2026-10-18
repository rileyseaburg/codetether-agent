//! Concrete tool-registry specialization of shared active model state.
pub(crate) type ModelState =
    codetether_request_preparation::model::ModelState<crate::tool::ToolRegistry>;
