use tetherscript::browser_agent::{BrowserPage, Locator, RouteAction, RoutePattern, RouteRule};
use tetherscript::js::JsValue;

#[test]
fn controlled_login_form_click_posts_without_native_navigation() {
    let html = "<form id='login' action='/native-login'>\
        <input id='email' name='email'><input id='password' name='password' type='password'>\
        <button id='submit' type='submit'>Sign in</button><div id='state'></div></form>\
        <script>\
        window.order='';let f=document.getElementById('login');let e=document.getElementById('email');\
        let p=document.getElementById('password');let b=document.getElementById('submit');\
        function track(x){window.order=window.order+x.type+'>';};\
        b.addEventListener('pointerdown',track);b.addEventListener('mousedown',track);\
        b.addEventListener('focus',track);b.addEventListener('pointerup',track);\
        b.addEventListener('mouseup',track);b.addEventListener('click',track);\
        function sync(){document.getElementById('state').textContent=e.value+'|'+p.value;};\
        e.addEventListener('input',sync);p.addEventListener('input',sync);\
        f.addEventListener('submit',function(x){x.preventDefault();\
        fetch('/api/login',{method:'post',headers:{'content-type':'application/json'},\
        body:JSON.stringify({email:e.value,password:p.value})});\
        document.getElementById('state').setAttribute('data-submitted','yes');});</script>";
    let mut page = BrowserPage::from_html("https://app.test/login", html);
    page.route(
        RouteRule::new(RoutePattern::substring("/api/login")),
        RouteAction::fulfill(200, "ok"),
    );

    page.run_scripts().unwrap();
    page.fill(&Locator::css("#email"), "agent@example.test")
        .unwrap();
    page.fill(&Locator::css("#password"), "secret").unwrap();
    page.click(&Locator::css("#submit")).unwrap();

    assert_eq!(page.session.url, "https://app.test/login");
    assert_eq!(
        page.eval_js("window.order").unwrap(),
        JsValue::String("pointerdown>mousedown>focus>pointerup>mouseup>click>".into())
    );
    assert_eq!(
        page.eval_js("document.getElementById('state').textContent")
            .unwrap(),
        JsValue::String("agent@example.test|secret".into())
    );
    let report = page.production_debug_report();
    let entry = report
        .network_har
        .iter()
        .find(|e| e.request.url.ends_with("/api/login"))
        .unwrap();
    assert_eq!(entry.request.method, "POST");
    assert!(entry
        .request
        .post_data
        .as_deref()
        .unwrap()
        .contains("agent@example.test"));
}

#[test]
fn controlled_login_form_enter_submits_containing_form() {
    let html = "<form id='login'><input id='email'><input id='password' type='password'>\
        <button id='submit'>Sign in</button></form><script>\
        let f=document.getElementById('login');let e=document.getElementById('email');\
        let p=document.getElementById('password');\
        f.addEventListener('submit',function(x){x.preventDefault();\
        fetch('/api/login',{method:'post',body:e.value+':'+p.value});});</script>";
    let mut page = BrowserPage::from_html("https://app.test/login", html);
    page.route(
        RouteRule::new(RoutePattern::substring("/api/login")),
        RouteAction::fulfill(200, "ok"),
    );

    page.run_scripts().unwrap();
    page.fill(&Locator::css("#email"), "agent@example.test")
        .unwrap();
    page.fill(&Locator::css("#password"), "secret").unwrap();
    page.press(&Locator::css("#password"), "Enter").unwrap();

    let report = page.production_debug_report();
    let entry = report
        .network_har
        .iter()
        .find(|e| e.request.url.ends_with("/api/login"))
        .unwrap();
    assert_eq!(
        entry.request.post_data.as_deref(),
        Some("agent@example.test:secret")
    );
}
