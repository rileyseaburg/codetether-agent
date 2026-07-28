use tetherscript::browser_agent::{BrowserPage, RouteAction, RoutePattern, RouteRule};
use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[path = "browser_js_metadata/navigator_capabilities.rs"]
mod navigator_capabilities;
#[path = "browser_js_metadata/navigator_connection.rs"]
mod navigator_connection;
#[path = "browser_js_metadata/navigator_user_agent_data.rs"]
mod navigator_user_agent_data;

#[test]
fn browser_identity_document_metadata_and_scroll_aliases_are_available() {
    let result = eval_with_dom(
        "<main></main>",
        "[navigator.webdriver,navigator.vendor,navigator.product,navigator.maxTouchPoints,\
         document.readyState,document.URL,document.documentURI,document.baseURI,\
         document.referrer,document.compatMode,scrollX,scrollY,pageXOffset,pageYOffset,\
         window.scrollX].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "false|TetherScript|Gecko|0|complete|http://localhost/|http://localhost/|\
             http://localhost/||CSS1Compat|0|0|0|0|0"
                .into()
        )
    );
}

#[test]
fn send_beacon_returns_true_and_logs_post_with_body_metadata() {
    let mut page = BrowserPage::from_html("https://app.test/", "<main></main>");
    page.route(
        RouteRule::new(RoutePattern::substring("/telemetry")),
        RouteAction::fulfill(202, "queued"),
    );

    let value = page
        .eval_js("navigator.sendBeacon('/telemetry','hello');")
        .unwrap();

    assert_eq!(value, JsValue::Bool(true));
    let event = page.session.network.last().unwrap();
    assert_eq!(event.method, "POST");
    assert_eq!(event.url, "/telemetry");
    assert_eq!(event.status, Some(202));
    assert_eq!(
        event.route_result.as_deref(),
        Some("beacon:body_bytes=5:fulfill")
    );
    assert_eq!(page.route_log().last().unwrap().method, "POST");
}
