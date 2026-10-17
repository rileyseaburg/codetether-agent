//! Runtime visibility into OS sandbox availability.

pub fn unavailable_reason() -> Option<&'static str> {
    match super::sandbox_runner_select::selected_runner() {
        super::sandbox_runner_select::Runner::Bubblewrap(_) => None,
        // Seatbelt (`sandbox-exec`) confines writes and network access on
        // macOS, so the sandbox is usable there without bwrap.
        super::sandbox_runner_select::Runner::Seatbelt(_) => None,
        // bwrap is unavailable, but Landlock can still confine a direct
        // process (no user namespace needed), so the sandbox is usable.
        super::sandbox_runner_select::Runner::Direct(_)
            if super::sandbox_landlock::kernel_available() =>
        {
            None
        }
        super::sandbox_runner_select::Runner::Direct(reason) => Some(reason),
    }
}

pub fn direct_fallback_env_allowed() -> bool {
    let value = std::env::var(super::sandbox_runner_direct::ENV).ok();
    super::sandbox_runner_direct::enabled(value.as_deref())
}
