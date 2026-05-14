const TEMPLATES: &[&str] = &[
    "live Argo proof requires app/job/pod identity plus success state",
    "Playwright proof requires command, exit state, and artifact path",
    "platform upload proof requires artifact path plus platform video ID",
    "DB migration proof requires environment, migration name, and row/query evidence",
    "deploy proof requires commit/image identity plus smoke-test evidence",
    "TetherScript parsers live at examples/tetherscript/evidence_ids.tether and evidence_argo.tether",
];

pub(crate) fn render() -> String {
    let mut out = String::from("Workflow evidence templates:");
    for template in TEMPLATES {
        out.push_str("\n- ");
        out.push_str(template);
    }
    out
}
