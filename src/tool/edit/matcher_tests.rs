use super::{apply, plan};

#[test]
fn supports_replace_all_for_repeated_exact_matches() {
    let content = "let a = 1;\nlet a = 1;\n";
    let plan = plan(content, "let a = 1;", true).unwrap();
    let (updated, replacements, backend) = apply(content, plan, "let a = 1;", "let a = 2;");
    assert_eq!(updated, "let a = 2;\nlet a = 2;\n");
    assert_eq!(replacements, 2);
    assert_eq!(backend, "exact_all");
}

#[test]
fn reports_closest_candidate_when_no_match_exists() {
    let content = "fn alpha() {\n    beta();\n}\n";
    let error = plan(content, "fn alpha() {\n    gamma();\n}", false).unwrap_err();
    assert_eq!(error["code"], "NOT_FOUND");
    assert_eq!(error["closest_candidate"], "fn alpha() {\n    beta();\n}");
}

#[test]
fn accepts_indentation_tolerant_match() {
    let content = "fn main() {\n  println!(\"hi\");\n}\n";
    let old = "fn main() {\n    println!(\"hi\");\n}";
    let plan = plan(content, old, false).unwrap();
    let (updated, _, backend) = apply(content, plan, old, "done");
    assert_eq!(updated, "done\n");
    assert_eq!(backend, "whitespace_tolerant");
}
