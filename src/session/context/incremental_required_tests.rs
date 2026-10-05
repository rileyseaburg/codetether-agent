//! Regression for a user instruction older than the recent tool-message window.
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

#[test]
fn keeps_active_instruction_but_leaves_old_tool_work_compressible() {
    let mut session = Session::default();
    session.messages = (0..20)
        .map(|idx| Message {
            role: if idx == 0 {
                Role::User
            } else {
                Role::Assistant
            },
            content: vec![ContentPart::Text {
                text: if idx == 0 {
                    "Implement the requested feature without changing its API".into()
                } else {
                    format!("tool work {idx}")
                },
            }],
        })
        .collect();
    let mut keep = vec![false; 20];
    let mut budget = 100;
    super::seed(&session, &mut keep, &[10; 20], 12, &mut budget);
    assert!(
        keep[0],
        "the instruction must not disappear after eight messages"
    );
    assert!(keep[1..12].iter().all(|kept| !kept));
    assert!(keep[12..].iter().all(|kept| *kept));
    assert_eq!(budget, 10);
}

#[test]
fn empty_history_keeps_its_budget() {
    let mut budget = 100;
    super::seed(&Session::default(), &mut [], &[], 0, &mut budget);
    assert_eq!(budget, 100);
}
