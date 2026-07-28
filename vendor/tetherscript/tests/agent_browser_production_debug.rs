use tetherscript::browser_agent::{
    BrowserPage, RouteAction, RouteFulfillment, RoutePattern, RouteRule, RuntimeExceptionKind,
};

#[test]
fn report_flags_react_errors_failed_requests_and_missing_source_maps() {
    let html = "<div id='root' data-reactroot></div><script src='/assets/app.js'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);
    page.register_script_resource(
        "/assets/app.js",
        "console.error('Hydration failed: text did not match');\
         fetch('/not-found');\
         //# sourceMappingURL=app.js.map",
    );

    page.run_scripts().unwrap();

    let report = page.production_debug_report();
    assert!(report.parity.native_engine);
    assert_eq!(report.console_errors.len(), 1);
    assert!(report
        .failed_requests
        .iter()
        .any(|entry| entry.contains("404")));
    assert!(report
        .runtime_exceptions
        .iter()
        .any(|item| item.kind == RuntimeExceptionKind::Network));
    assert_eq!(
        report
            .network_har
            .iter()
            .find(|entry| entry.request.url == "https://app.test/not-found")
            .unwrap()
            .response
            .status,
        404
    );
    assert!(report.frameworks.contains(&"react".to_string()));
    assert!(report.react.roots.contains(&"#root".to_string()));
    assert_eq!(report.react.hydration_warnings.len(), 1);
    assert_eq!(report.source_maps[0].source_map_url, "/assets/app.js.map");
    assert!(!report.source_maps[0].registered);
}

#[test]
fn report_exports_har_style_network_entries() {
    let mut page = BrowserPage::from_html("https://app.test/index.html", "<main></main>");
    page.route(
        RouteRule::new(RoutePattern::substring("/api/data")),
        RouteAction::Fulfill(RouteFulfillment {
            status: 201,
            headers: vec![("x-route".into(), "yes".into())],
            body: "{\"ok\":true}".into(),
        }),
    );

    page.eval_js("fetch('/api/data',{method:'post',headers:{'X-Test':'yes'},body:'payload'});")
        .unwrap();

    let report = page.production_debug_report();
    let entry = report
        .network_har
        .iter()
        .find(|entry| entry.request.url == "https://app.test/api/data")
        .unwrap();
    assert_eq!(entry.request.method, "POST");
    assert_eq!(entry.request.url, "https://app.test/api/data");
    assert_eq!(entry.request.headers, vec![("x-test".into(), "yes".into())]);
    assert_eq!(entry.request.post_data.as_deref(), Some("payload"));
    assert_eq!(entry.response.status, 201);
    assert_eq!(
        entry.response.headers,
        vec![("x-route".into(), "yes".into())]
    );
    assert_eq!(
        entry.response.content_text.as_deref(),
        Some("{\"ok\":true}")
    );
    assert_eq!(entry.response.route_result.as_deref(), Some("fulfill"));
}

#[test]
fn report_classifies_runtime_exceptions_for_agent_triage() {
    let mut page = BrowserPage::from_html("https://app.test/index.html", "<main></main>");

    page.eval_js("missing_call()").unwrap_err();
    page.eval_js("1()").unwrap_err();
    page.eval_js("let = ;").unwrap_err();
    page.eval_js("console.error('NotAllowedError: clipboard-read denied');")
        .unwrap();

    let report = page.production_debug_report();
    let kinds = report
        .runtime_exceptions
        .iter()
        .map(|item| item.kind)
        .collect::<Vec<_>>();
    assert!(kinds.contains(&RuntimeExceptionKind::Reference));
    assert!(kinds.contains(&RuntimeExceptionKind::Type));
    assert!(kinds.contains(&RuntimeExceptionKind::Syntax));
    assert!(kinds.contains(&RuntimeExceptionKind::Permission));
}

#[test]
fn report_accepts_registered_source_maps() {
    let html = "<div id='root'></div><script src='/assets/app.js'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);
    page.register_script_resource("/assets/app.js", "//# sourceMappingURL=app.js.map");
    page.register_source_map_resource("/assets/app.js.map", "{}");

    page.run_scripts().unwrap();

    let report = page.production_debug_report();
    assert!(report.source_maps[0].registered);
}

#[test]
fn report_remaps_runtime_errors_with_registered_source_maps() {
    let html = "<div id='root'></div><script src='/assets/app.js'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);
    page.register_script_resource(
        "/assets/app.js",
        "console.log('start'); missing_call();\n//# sourceMappingURL=app.js.map",
    );
    page.register_source_map_resource(
        "/assets/app.js.map",
        r#"{"version":3,"sources":["src/App.tsx"],"mappings":"sBASI","names":[]}"#,
    );

    let error = page.run_scripts().unwrap_err();
    assert!(error.contains("missing_call"));

    let report = page.production_debug_report();
    let mapped = &report.mapped_page_errors[0];
    assert_eq!(mapped.generated.script_url, "/assets/app.js");
    assert_eq!((mapped.generated.line, mapped.generated.column), (1, 23));
    let original = mapped.original.as_ref().unwrap();
    assert_eq!(original.source_url, "src/App.tsx");
    assert_eq!((original.line, original.column), (10, 5));
}

#[test]
fn report_remaps_generated_call_stack_frames() {
    let html = "<script src='/assets/app.js'></script>";
    let mut page = BrowserPage::from_html("https://app.test/index.html", html);
    page.register_script_resource(
        "/assets/app.js",
        "function c(){ missing_call(); } function b(){ c(); } function a(){ b(); } a();\
         \n//# sourceMappingURL=app.js.map",
    );
    page.register_source_map_resource(
        "/assets/app.js.map",
        r#"{"version":3,"sources":["src/App.tsx"],"mappings":"cA6BM,gCAVF,qBAVF","names":[]}"#,
    );

    page.run_scripts().unwrap_err();

    let report = page.production_debug_report();
    let stack = &report.mapped_page_errors[0].stack;
    let names = stack
        .iter()
        .map(|frame| frame.function_name.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(names, vec![Some("c"), Some("b"), Some("a")]);
    let originals = stack
        .iter()
        .map(|frame| {
            let loc = frame.original.as_ref().unwrap();
            (loc.source_url.as_str(), loc.line, loc.column)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        originals,
        vec![
            ("src/App.tsx", 30, 7),
            ("src/App.tsx", 20, 5),
            ("src/App.tsx", 10, 3),
        ]
    );
}
