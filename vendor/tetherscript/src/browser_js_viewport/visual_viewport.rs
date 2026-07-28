use super::*;

#[path = "visual_viewport_event_object.rs"]
mod event_object;
#[path = "visual_viewport_events.rs"]
mod events;
#[cfg(test)]
#[path = "tests_visual_viewport_events.rs"]
mod tests_visual_viewport_events;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    let object = Rc::new(RefCell::new(HashMap::new()));
    {
        let mut map = object.borrow_mut();
        install_metrics(&mut map);
    }
    events::install(&object);
    window.insert("visualViewport".into(), JsValue::Object(object));
}

fn install_metrics(map: &mut HashMap<String, JsValue>) {
    for (name, value) in [
        ("width", constants::DEFAULT_VIEWPORT_WIDTH as f64),
        ("height", constants::DEFAULT_VIEWPORT_HEIGHT as f64),
        ("scale", constants::DEVICE_PIXEL_RATIO),
        ("offsetLeft", 0.0),
        ("offsetTop", 0.0),
        ("pageLeft", 0.0),
        ("pageTop", 0.0),
    ] {
        map.insert(name.into(), JsValue::Number(value));
    }
}
