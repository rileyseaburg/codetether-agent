use super::*;
use std::cell::RefCell;
use std::rc::Rc;

const HANDLERS: [&str; 4] = [
    "onchargingchange",
    "onchargingtimechange",
    "ondischargingtimechange",
    "onlevelchange",
];

pub(super) fn install(navigator: &mut HashMap<String, JsValue>) {
    navigator.insert(
        "getBattery".into(),
        native("navigator.getBattery", Some(0), |_| {
            Ok(thenable::resolved(manager()))
        }),
    );
}

fn manager() -> JsValue {
    let mut battery = HashMap::new();
    battery.insert("charging".into(), JsValue::Bool(true));
    battery.insert("chargingTime".into(), JsValue::Number(0.0));
    battery.insert("dischargingTime".into(), JsValue::Number(f64::INFINITY));
    battery.insert("level".into(), JsValue::Number(1.0));
    for handler in HANDLERS {
        battery.insert(handler.into(), JsValue::Null);
    }
    events(&mut battery);
    JsValue::Object(Rc::new(RefCell::new(battery)))
}

fn events(battery: &mut HashMap<String, JsValue>) {
    battery.insert(
        "addEventListener".into(),
        native("BatteryManager.addEventListener", None, |_| {
            Ok(JsValue::Undefined)
        }),
    );
    battery.insert(
        "removeEventListener".into(),
        native("BatteryManager.removeEventListener", None, |_| {
            Ok(JsValue::Undefined)
        }),
    );
    battery.insert(
        "dispatchEvent".into(),
        native("BatteryManager.dispatchEvent", Some(1), |_| {
            Ok(JsValue::Bool(true))
        }),
    );
}
