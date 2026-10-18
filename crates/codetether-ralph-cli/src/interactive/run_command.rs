//! Run validation, provider selection and interactive launch updates.
use super::{Host, Registry, View, parse_ralph_run_args, resolve_prd_path, spawn_ralph_run};
use std::{path::Path, sync::Arc};
pub(super) fn run<H: Host, R: Registry, V: View>(
    view: &mut V,
    cwd: &Path,
    model: Option<&str>,
    registry: Option<&Arc<R>>,
    args: &[&str],
) -> bool {
    let (prd_arg, max_iters) = parse_ralph_run_args(args);
    let Some(registry) = registry.cloned() else {
        view.set_status("Ralph run failed: no provider registry available".to_string());
        return true;
    };
    let prd_path = resolve_prd_path(cwd, prd_arg);
    if !prd_path.exists() {
        view.set_status(format!(
            "Ralph run failed: PRD not found at {}",
            prd_path.display()
        ));
        return true;
    }
    let model_str = model.unwrap_or("claude-sonnet-4-5");
    let (provider, model) = match registry.resolve_model(model_str) {
        Ok(pair) => pair,
        Err(err) => {
            view.set_status(format!("Ralph run failed: {err}"));
            return true;
        }
    };
    let (tx, rx) = tokio::sync::mpsc::channel(256);
    view.attach_event_rx(rx);
    view.show_monitor();
    view.set_status(format!(
        "Ralph running: {} (max {max_iters} iterations)",
        prd_path.display()
    ));
    view.push_system_message(format!(
        "Launching Ralph on `{}` via model `{model}` (max {max_iters} iterations).",
        prd_path.display()
    ));
    spawn_ralph_run::<H>(prd_path, provider, model, max_iters, tx);
    true
}
