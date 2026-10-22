use crate::strip_stats_header;

#[test]
fn strips_router_banner() {
    assert_eq!(strip_stats_header("[RLM: x]\n\nbody"), "body");
}

#[test]
fn strips_engine_banner_without_removing_summary_sections() {
    let body = "## ESTABLISHED EVIDENCE\ncall(id)";
    assert_eq!(
        strip_stats_header(&format!(
            "[RLM engine: 100 -> 10 tokens | evidence-first]\n\n{body}"
        )),
        body
    );
    assert_eq!(strip_stats_header(body), body);
}
