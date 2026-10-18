//! Application tool and agent-loop selection for the native reviewer.
pub(super) type Native = codetether_native_review::Reviewer<
    crate::tool::registry_lifecycle::Native,
    crate::swarm::executor::LoopHost,
>;
