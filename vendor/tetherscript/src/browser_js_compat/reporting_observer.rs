use super::super::*;

#[cfg(test)]
#[path = "tests_reporting_observer.rs"]
mod tests;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "ReportingObserver".into(),
        native("ReportingObserver", None, construct),
    );
}

fn construct(args: &[JsValue]) -> Result<JsValue, String> {
    let callback = args.first().cloned().unwrap_or(JsValue::Undefined);
    let map = Rc::new(RefCell::new(HashMap::new()));
    let observer = JsValue::Object(map.clone());
    {
        let mut obj = map.borrow_mut();
        obj.insert("observe".into(), observe(callback, observer.clone()));
        obj.insert("disconnect".into(), disconnect());
        obj.insert("takeRecords".into(), take_records());
    }
    Ok(observer)
}

fn observe(callback: JsValue, observer: JsValue) -> JsValue {
    native("ReportingObserver.observe", None, move |_| {
        if !matches!(&callback, JsValue::Undefined | JsValue::Null) {
            let _ = js::call_function_with_this(
                callback.clone(),
                JsValue::Undefined,
                &[empty_reports(), observer.clone()],
            );
        }
        Ok(JsValue::Undefined)
    })
}

fn disconnect() -> JsValue {
    native("ReportingObserver.disconnect", None, |_| {
        Ok(JsValue::Undefined)
    })
}

fn take_records() -> JsValue {
    native("ReportingObserver.takeRecords", None, |_| {
        Ok(empty_reports())
    })
}

fn empty_reports() -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(Vec::new())))
}
