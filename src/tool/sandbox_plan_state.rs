use super::{sandbox_landlock, sandbox_runner, sandbox_seccomp};

pub(super) struct PlanState {
    pub program: String,
    pub args: Vec<String>,
    pub unsafe_fallbacks: Vec<String>,
    pub landlock: Option<sandbox_landlock::Rules>,
    pub _seccomp: Option<sandbox_seccomp::Program>,
}

pub(super) fn from_plan(
    plan: sandbox_runner::RunnerPlan,
    network_fallbacks: Vec<String>,
) -> PlanState {
    let sandbox_runner::RunnerPlan {
        program,
        args,
        unsafe_fallbacks,
        network_isolated,
        _seccomp,
        landlock,
    } = plan;
    PlanState {
        program,
        args,
        unsafe_fallbacks: fallbacks(network_isolated, network_fallbacks, unsafe_fallbacks),
        landlock,
        _seccomp,
    }
}

fn fallbacks(
    network_isolated: bool,
    network_fallbacks: Vec<String>,
    plan_fallbacks: Vec<String>,
) -> Vec<String> {
    let mut out = Vec::new();
    if !network_isolated {
        out.extend(network_fallbacks);
    }
    out.extend(plan_fallbacks);
    out
}
