//! Runtime visibility into OS sandbox availability.

pub fn unavailable_reason() -> Option<&'static str> {
    unavailable_reason_for(crate::tool::network_access::allowed())
}

pub(crate) fn unavailable_reason_for(allow_network: bool) -> Option<&'static str> {
    match super::sandbox_runner_select::selected_runner() {
        super::sandbox_runner_select::Runner::Bubblewrap(_) => None,
        // Seatbelt (`sandbox-exec`) confines writes and network access on
        // macOS, so the sandbox is usable there without bwrap.
        super::sandbox_runner_select::Runner::Seatbelt(_) => None,
        // Landlock confines files while direct seccomp blocks privileged and
        // networking syscalls without requiring a user namespace.
        super::sandbox_runner_select::Runner::Direct(_)
            if super::sandbox_landlock::kernel_available()
                && super::sandbox_seccomp::prepare(allow_network).is_ok_and(|value| value.is_some()) =>
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