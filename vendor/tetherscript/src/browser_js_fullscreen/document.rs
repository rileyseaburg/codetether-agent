use super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>, handle: &DomHandle) {
    let (fullscreen, pointer) = state::snapshot();
    props::write(obj, fullscreen.as_ref(), pointer.as_ref());
    let h = handle.clone();
    obj.insert(
        "exitFullscreen".into(),
        native("Document.exitFullscreen", Some(0), move |_| {
            let previous = state::exit_fullscreen();
            documents::refresh();
            if let Some(target) = previous {
                events::change_or_document(&target.handle, &h, "fullscreenchange")?;
            }
            Ok(thenable::resolved(JsValue::Undefined))
        }),
    );
    let h = handle.clone();
    obj.insert(
        "exitPointerLock".into(),
        native("Document.exitPointerLock", Some(0), move |_| {
            let previous = state::exit_pointer_lock();
            documents::refresh();
            if let Some(target) = previous {
                events::change_or_document(&target.handle, &h, "pointerlockchange")?;
            }
            Ok(JsValue::Undefined)
        }),
    );
}

pub(super) fn refresh(
    document: &DomObject,
    fullscreen: Option<&target::Target>,
    pointer: Option<&target::Target>,
) {
    props::write(&mut document.borrow_mut(), fullscreen, pointer);
}
