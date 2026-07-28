use super::*;

pub(super) fn install(main: &JsObject, worker: JsObject, timers: Rc<RefCell<TimerQueue>>) {
    let post_worker = worker.clone();
    main.borrow_mut().insert(
        "postMessage".into(),
        native("Worker.postMessage", Some(1), move |args| {
            if !closed(&post_worker) {
                let data = args.first().cloned().unwrap_or(JsValue::Undefined);
                port::queue(
                    timers.clone(),
                    post_worker.clone(),
                    data,
                    String::new(),
                    JsValue::Null,
                );
            }
            Ok(JsValue::Undefined)
        }),
    );
    let main_object = main.clone();
    main.borrow_mut().insert(
        "terminate".into(),
        native("Worker.terminate", Some(0), move |_| {
            mark_closed(&main_object);
            mark_closed(&worker);
            Ok(JsValue::Undefined)
        }),
    );
}
