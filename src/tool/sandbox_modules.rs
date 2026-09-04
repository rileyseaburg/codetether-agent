// Submodule wiring for the sandbox module, kept separate so `sandbox.rs`
// stays focused on plugin verification and policy types.
#[path = "sandbox_availability.rs"]
mod sandbox_availability;
#[path = "sandbox_bwrap_args.rs"]
mod sandbox_bwrap_args;
#[path = "sandbox_bwrap_paths.rs"]
mod sandbox_bwrap_paths;
#[path = "sandbox_bwrap_probe.rs"]
mod sandbox_bwrap_probe;
#[path = "sandbox_bwrap_push.rs"]
mod sandbox_bwrap_push;
#[path = "sandbox_command.rs"]
mod sandbox_command;
#[path = "sandbox_env.rs"]
mod sandbox_env;
#[path = "sandbox_execute.rs"]
mod sandbox_execute;
#[path = "sandbox_landlock.rs"]
mod sandbox_landlock;
#[path = "sandbox_plan_state.rs"]
mod sandbox_plan_state;
include!("sandbox_process_modules.rs");
#[path = "sandbox_result_builder.rs"]
mod sandbox_result_builder;
#[path = "sandbox_runner.rs"]
mod sandbox_runner;
#[path = "sandbox_runner_bwrap.rs"]
mod sandbox_runner_bwrap;
#[path = "sandbox_runner_direct.rs"]
mod sandbox_runner_direct;
#[path = "sandbox_runner_seatbelt.rs"]
mod sandbox_runner_seatbelt;
#[path = "sandbox_runner_select.rs"]
mod sandbox_runner_select;
#[path = "sandbox_seatbelt.rs"]
mod sandbox_seatbelt;
#[path = "sandbox_seccomp.rs"]
mod sandbox_seccomp;
#[path = "sandbox_toolchain.rs"]
mod sandbox_toolchain;
