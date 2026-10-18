//! Application specialization of the shared agent-management tool.
codetether_agent_tool::declare_application_tool!(
    crate::tool::agent::persistence,
    crate::tool::agent::spawn,
    crate::tool::agent::message,
    crate::tool::agent::handlers,
    crate::tool::agent::status,
    crate::tool::agent::thread_lifecycle,
);
