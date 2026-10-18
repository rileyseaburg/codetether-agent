//! Managed-session contracts using the native prior-context policy.
codetether_agent_session::impl_managed_session!(
    crate::session::Session,
    crate::session::helper::runtime::prior_context_allowed_for_session
);
