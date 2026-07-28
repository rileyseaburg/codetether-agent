use super::*;

const MAX_READ_BYTES: usize = 1024 * 1024;

pub(super) fn install(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: events::ListenerList,
    method: &str,
    native_name: &'static str,
    kind: kind::ReadKind,
) {
    let reader = object.clone();
    object.borrow_mut().insert(
        method.into(),
        native(native_name, Some(1), move |args| {
            read(&reader, &listeners, args, kind)
        }),
    );
}

fn read(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    listeners: &events::ListenerList,
    args: &[JsValue],
    kind: kind::ReadKind,
) -> Result<JsValue, String> {
    let input = args.first().unwrap_or(&JsValue::Undefined);
    let data = blob::bytes(input).unwrap_or_else(|| input.display().into_bytes());
    state::start(object);
    events::dispatch(object, listeners, "loadstart")?;
    if !state::is_loading(object) {
        return Ok(JsValue::Undefined);
    }
    if data.len() > MAX_READ_BYTES {
        return state::fail(object, listeners, "NotReadableError");
    }
    state::finish(object, listeners, kind.result(input, &data))
}
