use super::super::*;

pub(super) fn install(obj: &mut HashMap<String, JsValue>) {
    obj.insert("hidden".into(), JsValue::Bool(false));
    obj.insert("visibilityState".into(), JsValue::String("visible".into()));
    obj.insert("currentScript".into(), JsValue::Null);
    obj.insert(
        "hasFocus".into(),
        native("document.hasFocus", Some(0), move |_| {
            Ok(JsValue::Bool(true))
        }),
    );
}

#[cfg(test)]
mod tests {
    use crate::browser_js::eval_with_dom;
    use crate::js::JsValue;

    #[test]
    fn document_visibility_and_focus_metadata_are_stable() {
        let result = eval_with_dom(
            "<main></main>",
            "document.hidden+':'+document.visibilityState+':'\
             +document.hasFocus()+':'+document.currentScript;",
        )
        .unwrap();
        assert_eq!(
            result.value,
            JsValue::String("false:visible:true:null".into())
        );
    }
}
