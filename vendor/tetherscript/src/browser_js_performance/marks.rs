//! User-timing mark and measure methods.

use super::*;

pub(super) fn install(performance: &mut HashMap<String, JsValue>, timers: Rc<RefCell<TimerQueue>>) {
    let mark_timers = timers.clone();
    performance.insert(
        "mark".into(),
        native("performance.mark", Some(1), move |args| {
            let entry = state::add_mark(string_arg(args, 0));
            observer::notify(entry.clone(), mark_timers.clone());
            Ok(entry.to_js())
        }),
    );
    performance.insert(
        "measure".into(),
        native("performance.measure", None, move |args| {
            let entry = state::add_measure(
                string_arg(args, 0),
                optional_name(args.get(1)),
                optional_name(args.get(2)),
            )?;
            observer::notify(entry.clone(), timers.clone());
            Ok(entry.to_js())
        }),
    );
}

fn string_arg(args: &[JsValue], index: usize) -> String {
    args.get(index)
        .map(JsValue::display)
        .unwrap_or_else(|| "".into())
}

fn optional_name(value: Option<&JsValue>) -> Option<String> {
    match value {
        Some(JsValue::Undefined | JsValue::Null) | None => None,
        Some(value) => Some(value.display()),
    }
}
