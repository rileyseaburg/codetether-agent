use crate::browser::{
    BrowserError, BrowserOutput,
    request::{KeyboardPressRequest, KeyboardTypeRequest},
};

pub(super) async fn keyboard_type(
    session: &super::super::super::BrowserSession,
    request: KeyboardTypeRequest,
) -> Result<BrowserOutput, BrowserError> {
    let text = serde_json::to_string(&request.text).unwrap_or_else(|_| "\"\"".into());
    let script = format!(
        "let el=document.activeElement;if(!el)false;else{{el.value=(el.value||'')+{text};el.dispatchEvent({{type:'input'}});true}}"
    );
    super::page::with(session, |page| {
        page.eval_js(&script)
            .map_err(super::super::eval::js_error)?;
        Ok(())
    })
    .await
}

pub(super) async fn keyboard_press(
    session: &super::super::super::BrowserSession,
    request: KeyboardPressRequest,
) -> Result<BrowserOutput, BrowserError> {
    let key = serde_json::to_string(&request.key).unwrap_or_else(|_| "\"\"".into());
    let script = format!(
        "let el=document.activeElement||document.body;el.dispatchEvent({{type:'keydown',key:{key}}});el.dispatchEvent({{type:'keyup',key:{key}}});"
    );
    super::page::with(session, |page| {
        page.eval_js(&script)
            .map_err(super::super::eval::js_error)?;
        Ok(())
    })
    .await
}
