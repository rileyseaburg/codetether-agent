use super::super::*;

#[path = "tests_url_static_object_url.rs"]
mod object_url;

#[test]
fn url_can_parse_accepts_absolute_and_based_relative_urls() {
    let result = eval_with_dom(
        "<main></main>",
        "URL.canParse('https://example.test/a') + ':' +\
         URL.canParse('/a?q=1', 'https://example.test/base/page') + ':' +\
         URL.canParse('child', 'https://example.test/base/page') + ':' +\
         URL.canParse('') + ':' + URL.canParse('/a') + ':' +\
         URL.canParse('/a', 'not-a-url');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("true:true:true:false:false:false".into())
    );
}

#[test]
fn url_parse_returns_object_or_null() {
    let result = eval_with_dom(
        "<main></main>",
        "let absolute=URL.parse('https://example.test/a?q=1#h');\
         let relative=URL.parse('/child?q=2', 'https://example.test/base/page');\
         let bad=URL.parse('');\
         absolute.host + ':' + absolute.pathname + ':' + absolute.searchParams.get('q') + '|' +\
         relative.href + ':' + relative.pathname + ':' + relative.searchParams.get('q') + '|' +\
         (bad === null);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("example.test:/a:1|https://example.test/child?q=2:/child:2|true".into())
    );
}

#[test]
fn location_reload_is_present_and_keeps_href() {
    let result = eval_with_dom(
        "<main></main>",
        "location.assign('/ready#top');let before=location.href;\
         let returned=location.reload();\
         before+'|'+location.href+'|'+returned;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("/ready#top|/ready#top|undefined".into())
    );
}
