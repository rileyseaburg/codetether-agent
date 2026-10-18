//! Session work-item tools using native goal journals and verification.
/// Session task tool bound to the application's existing verifier.
pub type SessionTaskTool =
    codetether_session_task::SessionTaskTool<crate::tool::goal::host::Native>;
