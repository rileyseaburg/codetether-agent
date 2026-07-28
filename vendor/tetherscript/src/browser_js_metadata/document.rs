//! Static document metadata for app compatibility checks.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::js::JsValue;

use super::{href, set_str};

#[path = "document_policy.rs"]
mod policy;

pub(super) fn install(document: &JsValue, location: &Rc<RefCell<HashMap<String, JsValue>>>) {
    let JsValue::Object(document) = document else {
        return;
    };
    let href = href(location);
    let mut document = document.borrow_mut();
    document.insert("location".into(), JsValue::Object(location.clone()));
    set_str(&mut document, "readyState", "complete");
    set_str(&mut document, "URL", href.clone());
    set_str(&mut document, "documentURI", href.clone());
    set_str(&mut document, "baseURI", href);
    set_str(&mut document, "referrer", "");
    set_str(&mut document, "compatMode", "CSS1Compat");
    set_str(&mut document, "characterSet", "UTF-8");
    set_str(&mut document, "charset", "UTF-8");
    set_str(&mut document, "contentType", "text/html");
    set_str(&mut document, "lastModified", "01/01/1970 00:00:00");
    document.insert("hidden".into(), JsValue::Bool(false));
    set_str(&mut document, "visibilityState", "visible");
    document.insert("prerendering".into(), JsValue::Bool(false));
    document.insert("featurePolicy".into(), policy::object("featurePolicy"));
    document.insert(
        "permissionsPolicy".into(),
        policy::object("permissionsPolicy"),
    );
}
