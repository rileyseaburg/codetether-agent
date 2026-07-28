//! Page understanding unit tests.

use super::*;

#[test]
fn detects_regions() {
    let els = vec![ElementSummary {
        selector: "nav".into(),
        tag: "nav".into(),
        ..Default::default()
    }];
    let regions = regions::detect_regions(&els);
    assert_eq!(regions[0].kind, PageRegion::Navigation);
}

#[test]
fn classifies_login_form() {
    let inputs = vec![
        InputSummary {
            input_type: "email".into(),
            name: Some("email".into()),
            ..Default::default()
        },
        InputSummary {
            input_type: "password".into(),
            name: Some("password".into()),
            ..Default::default()
        },
    ];
    assert_eq!(forms::classify_form(&inputs), FormPurpose::Login);
}

#[test]
fn classifies_download_link() {
    assert_eq!(
        links::classify_link("/report.pdf", "Download report", "main"),
        LinkKind::Download
    );
}

#[test]
fn detects_actionable_button() {
    let els = vec![ElementSummary {
        selector: "#save".into(),
        tag: "button".into(),
        text: "Save".into(),
        ..Default::default()
    }];
    let actions = actions::detect_actionable(&els);
    assert_eq!(actions[0].kind, "button");
}
