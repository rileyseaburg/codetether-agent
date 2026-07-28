use super::super::*;

#[test]
fn url_pattern_string_and_object_patterns_match_deterministically() {
    let result = eval_with_dom(
        "<main></main>",
        "let a=new URLPattern('https://*.example.test/users/*?q=*#top');\
         let b=URLPattern({hostname:'example.test',pathname:'/docs/*'});\
         [typeof URLPattern,window.URLPattern===URLPattern,a.protocol,a.hostname,\
         a.pathname,a.search,a.hash,\
         a.test('https://api.example.test/users/42?q=1#top'),\
         a.test('https://api.example.test/admin?q=1#top'),\
         b.protocol,b.search,b.hash,\
         b.test('https://example.test/docs/page?q=1#s'),\
         b.test('https://other.test/docs/page')].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "function|true|https|*.example.test|/users/*|q=*|top|true|false|*|*|*|true|false"
                .into()
        )
    );
}

#[test]
fn url_pattern_exec_reports_inputs_and_component_groups() {
    let result = eval_with_dom(
        "<main></main>",
        "let p=URLPattern('/app/*','https://site.test/base/page');\
         let hit=p.exec('/app/route?q=1','https://site.test/start');\
         let miss=p.exec('/app/route','https://other.test/start');\
         [p.protocol,p.hostname,p.pathname,p.search,p.hash,p.test('/app/x','https://site.test/'),\
         miss===null,hit.inputs.join(','),hit.hostname.input,hit.pathname.input,\
         hit.search.input,typeof hit.pathname.groups].join('|');",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String(
            "https|site.test|/app/*|*|*|true|true|/app/route?q=1,https://site.test/start|site.test|/app/route|q=1|object"
                .into()
        )
    );
}
