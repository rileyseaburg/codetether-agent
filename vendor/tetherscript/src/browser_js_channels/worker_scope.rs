use super::*;

pub(super) fn object(main: JsObject, url: &str, timers: Rc<RefCell<TimerQueue>>) -> JsObject {
    let origin = url.to_string();
    let object = Rc::new(RefCell::new(HashMap::from([
        ("location".into(), JsValue::String(url.into())),
        ("onmessage".into(), JsValue::Null),
        ("__closed".into(), JsValue::Bool(false)),
    ])));
    listeners::install(&object, "WorkerGlobalScope");
    object.borrow_mut().insert(
        "postMessage".into(),
        native("WorkerGlobalScope.postMessage", Some(1), move |args| {
            if !closed(&main) {
                let data = args.first().cloned().unwrap_or(JsValue::Undefined);
                port::queue(
                    timers.clone(),
                    main.clone(),
                    data,
                    origin.clone(),
                    JsValue::Null,
                );
            }
            Ok(JsValue::Undefined)
        }),
    );
    object
}

pub(super) fn run(worker: &JsObject, source: &str) -> Result<(), String> {
    let mut engine = JsEngine::new();
    engine.set_global("self", JsValue::Object(worker.clone()));
    for name in ["postMessage", "addEventListener", "removeEventListener"] {
        if let Some(value) = worker.borrow().get(name).cloned() {
            engine.set_global(name, value);
        }
    }
    engine.eval(source).map(|_| ())
}
