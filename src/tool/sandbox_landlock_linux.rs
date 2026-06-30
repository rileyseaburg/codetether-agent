#[path = "sandbox_landlock_linux_apply.rs"]
mod apply_impl;
#[path = "sandbox_landlock_linux_plan.rs"]
mod plan;
#[path = "sandbox_landlock_linux_restrict.rs"]
mod restrict;
#[path = "sandbox_landlock_linux_sys.rs"]
mod sys;

pub(crate) use plan::{Rules, prepare};

pub(crate) fn apply(cmd: &mut tokio::process::Command, rules: Option<Rules>) {
    apply_impl::apply(cmd, rules);
}

/// Landlock ABI version reported by the kernel (`<= 0` means unavailable).
pub(crate) fn abi_version() -> i64 {
    sys::abi_version()
}
