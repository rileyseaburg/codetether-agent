//! Window performance object assembly.

use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    let mut performance = HashMap::new();
    performance.insert("timeOrigin".into(), JsValue::Number(0.0));
    performance.insert(
        "now".into(),
        native("performance.now", Some(0), |_| {
            Ok(JsValue::Number(state::now()))
        }),
    );
    performance.insert(
        "toJSON".into(),
        native("performance.toJSON", Some(0), |_| Ok(snapshot())),
    );
    marks::install(&mut performance, timers.clone());
    read::install(&mut performance);
    clear::install(&mut performance);
    window.insert(
        "performance".into(),
        JsValue::Object(Rc::new(RefCell::new(performance))),
    );
}

fn snapshot() -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(HashMap::from([(
        "timeOrigin".into(),
        JsValue::Number(0.0),
    )]))))
}
