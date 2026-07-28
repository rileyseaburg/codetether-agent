use super::*;

thread_local! {
    static QUEUE_MICROTASK: RefCell<Option<JsValue>> = const { RefCell::new(None) };
}

pub(super) fn install(queue_microtask: Option<JsValue>) {
    QUEUE_MICROTASK.with(|queue| *queue.borrow_mut() = queue_microtask);
}

pub(super) fn reset() {
    QUEUE_MICROTASK.with(|queue| *queue.borrow_mut() = None);
}

pub(super) fn enqueue(job: JsValue) {
    let queued = QUEUE_MICROTASK.with(|queue| {
        queue.borrow().clone().is_some_and(|callback| {
            js::call_function_with_this(callback, JsValue::Undefined, std::slice::from_ref(&job))
                .is_ok()
        })
    });
    if !queued {
        let _ = js::call_function_with_this(job, JsValue::Undefined, &[]);
    }
}
