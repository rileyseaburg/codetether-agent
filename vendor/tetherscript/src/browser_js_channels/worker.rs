use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    window.insert(
        "registerWorkerScript".into(),
        native("registerWorkerScript", Some(2), move |args| {
            let url = args.first().unwrap_or(&JsValue::Undefined).display();
            let source = args.get(1).unwrap_or(&JsValue::Undefined).display();
            worker_scripts::register(&url, &source);
            Ok(JsValue::Undefined)
        }),
    );
    window.insert(
        "Worker".into(),
        native("Worker", Some(1), move |args| {
            let url = args.first().unwrap_or(&JsValue::Undefined).display();
            let source = worker_scripts::get(&url)
                .ok_or_else(|| format!("Worker: no registered script for {url}"))?;
            worker_host::spawn(&url, &source, timers.clone())
        }),
    );
}
