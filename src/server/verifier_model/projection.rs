//! Use verifier precedence for configuration, separately from observed execution.

use super::{defaults::Defaults, types::Settings};
use crate::tool::goal::verify::{observation::Observation, select_verifier_model};

fn clean(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub(super) fn snapshot(
    runtime: Option<String>,
    defaults: Defaults,
    worker: Option<String>,
    latest: Option<Observation>,
) -> Settings {
    let runtime = clean(runtime);
    let environment = clean(defaults.environment);
    let configured = clean(defaults.configured);
    let worker = clean(worker);
    let selected = select_verifier_model(
        runtime.as_deref().or(environment.as_deref()),
        worker.as_deref(),
        configured.as_deref(),
    );
    let source = super::source::classify(
        runtime.as_deref(),
        environment.as_deref(),
        configured.as_deref(),
        worker.as_deref(),
    );
    Settings {
        identity_source: "harness_configuration",
        scope: "process",
        persisted: false,
        runtime_model: runtime,
        environment_model: environment,
        default_model: configured,
        worker_model_context: worker,
        selected_model: selected,
        source,
        latest_verification: latest,
    }
}
