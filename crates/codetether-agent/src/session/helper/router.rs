//! Shared proactive diagnostics and provider failover selection.
pub use codetether_application_registry::proactive::build_proactive_lsp_context_message;
pub use codetether_model_failover::router::{
    choose_router_target, choose_router_target_bandit, known_good_router_candidates,
};
