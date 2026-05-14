use crate::browser::{BrowserError, BrowserOutput, request::PointerClick};

pub(super) async fn mouse_click(
    session: &super::super::super::BrowserSession,
    request: PointerClick,
) -> Result<BrowserOutput, BrowserError> {
    let script = format!(
        "let el=document.elementFromPoint({},{});if(el){{el.click();true}}else false;",
        request.x, request.y
    );
    super::page::with(session, |page| {
        if page.eval_js(&script).map_err(js_error)?.truthy() {
            Ok(())
        } else {
            Err(BrowserError::ElementNotFound(format!(
                "point {},{}",
                request.x, request.y
            )))
        }
    })
    .await
}

fn js_error(message: String) -> BrowserError {
    super::super::eval::js_error(message)
}
