//! linux for Landlock confinement.
#[path = "linux_apply.rs"]
mod apply_impl;
#[path = "linux_plan.rs"]
mod plan;
#[path = "linux_restrict.rs"]
mod restrict;
#[path = "linux_sys.rs"]
mod sys;

pub use plan::{Rules, prepare};

/// Register pre-exec confinement for the supplied rules.
/// No restrictions are applied to the calling process.
pub fn apply(cmd: &mut tokio::process::Command, rules: Option<Rules>) {
    apply_impl::apply(cmd, rules);
}

/// Landlock ABI version reported by the kernel (`<= 0` means unavailable).
pub fn abi_version() -> i64 {
    sys::abi_version()
}
