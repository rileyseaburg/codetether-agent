//! Detect shell Git operations that bypass repository hooks.
#[path = "git_hooks/clean.rs"]
mod clean_impl;
#[path = "git_hooks/detected.rs"]
mod detected_impl;
#[path = "git_hooks/executable.rs"]
mod executable_impl;
#[path = "git_hooks/hook_skip_env.rs"]
mod hook_skip_env_impl;
#[path = "git_hooks/hooks_config.rs"]
mod hooks_config_impl;
#[path = "git_hooks/segment_bypasses_hooks.rs"]
mod segment_bypasses_hooks_impl;
#[path = "git_hooks/short_n.rs"]
mod short_n_impl;
#[path = "git_hooks/subcommand_bypasses.rs"]
mod subcommand_bypasses_impl;
use clean_impl::clean;
pub(super) use detected_impl::detected;
use executable_impl::executable;
use hook_skip_env_impl::hook_skip_env;
use hooks_config_impl::hooks_config;
use segment_bypasses_hooks_impl::segment_bypasses_hooks;
use short_n_impl::short_n;
use subcommand_bypasses_impl::subcommand_bypasses;
