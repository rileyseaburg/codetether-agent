use super::super::run;

#[test]
fn json_flag_compacts_json_output() {
    let output = "{\n  \"ok\": true\n}";
    assert_eq!(run::format_output(output, true), "{\"ok\":true}");
    assert_eq!(run::format_output("not-json", true), "not-json");
}
