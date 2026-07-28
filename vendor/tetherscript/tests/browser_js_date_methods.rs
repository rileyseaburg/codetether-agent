use tetherscript::js;

#[test]
fn date_getters_and_setters_cover_bundle_archive_helpers() {
    let source = "let d=new Date(0);d.setFullYear(2020);d.setMonth(1);\
        d.setDate(3);d.setHours(4);d.setMinutes(5);d.setSeconds(6);\
        d.setMilliseconds(7);d.getFullYear()+':'+d.getMonth()+':'+d.getDate()+\
        ':'+d.getHours()+':'+d.getMinutes()+':'+d.getSeconds()+':'+\
        d.getMilliseconds()+':'+d.getTimezoneOffset()+':'+d.toISOString();";

    assert_eq!(
        js::eval(source).unwrap(),
        js::JsValue::String("2020:1:3:4:5:6:7:0:2020-02-03T04:05:06.007Z".into())
    );
}
