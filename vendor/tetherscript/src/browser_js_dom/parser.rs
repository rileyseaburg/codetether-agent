use super::*;

pub(super) fn install(window: &mut HashMap<String, JsValue>) {
    window.insert(
        "DOMParser".into(),
        native("DOMParser", Some(0), move |_| Ok(parser_object())),
    );
}

fn parser_object() -> JsValue {
    let mut obj = HashMap::new();
    obj.insert(
        "parseFromString".into(),
        native("DOMParser.parseFromString", Some(2), move |args| {
            let markup = args.first().unwrap_or(&JsValue::Undefined).display();
            let mime = args.get(1).unwrap_or(&JsValue::Undefined).display();
            let document = if is_html_mime(&mime) {
                construct::document_from_html(&markup)
            } else if is_xml_mime(&mime) {
                construct::document_from_markup(&markup)
            } else {
                construct::document_from_html("")
            };
            Ok(document)
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn is_html_mime(mime: &str) -> bool {
    mime.trim().eq_ignore_ascii_case("text/html")
}

fn is_xml_mime(mime: &str) -> bool {
    matches!(
        mime.trim().to_ascii_lowercase().as_str(),
        "image/svg+xml" | "application/xml" | "text/xml"
    )
}
