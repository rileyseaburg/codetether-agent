use super::*;

pub(super) fn spawn(
    url: &str,
    source: &str,
    timers: Rc<RefCell<TimerQueue>>,
) -> Result<JsValue, String> {
    let main = Rc::new(RefCell::new(HashMap::from([
        ("url".into(), JsValue::String(url.into())),
        ("onmessage".into(), JsValue::Null),
        ("__closed".into(), JsValue::Bool(false)),
    ])));
    let worker = worker_scope::object(main.clone(), url, timers.clone());
    listeners::install(&main, "Worker");
    worker_main::install(&main, worker.clone(), timers);
    worker_scope::run(&worker, source)?;
    Ok(JsValue::Object(main))
}
