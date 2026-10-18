//! Serialize process environment and reset shared test grants.
use codetether_config::Config;
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
pub(crate) fn lock_env() -> std::sync::MutexGuard<'static, ()> {
    let guard = ENV_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    codetether_approval_core::session_grants::reset();
    codetether_approval_core::session_command_grants::reset();
    Config::apply_process_access_mode_override(None);
    guard
}
