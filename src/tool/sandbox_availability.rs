//! Runtime visibility into OS sandbox availability.

pub fn unavailable_reason() -> Option<&'static str> {
    match super::sandbox_runner_select::selected_runner() {
        super::sandbox_runner_select::Runner::Bubblewrap(_) => None,
        super::sandbox_runner_select::Runner::Direct(reason) => Some(reason),
    }
}

pub fn direct_fallback_env_allowed() -> bool {
    let value = std::env::var(super::sandbox_runner_direct::ENV).ok();
    super::sandbox_runner_direct::enabled(value.as_deref())
}
