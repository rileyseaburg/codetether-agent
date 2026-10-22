use codetether_context_policy::{
    pages::PageKind,
    state_header::{ConstraintRecord, prepend_state_header},
};
use codetether_provider_core::{ContentPart, Message, Role};

fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
fn render(
    records: Vec<ConstraintRecord>,
    pages: &[PageKind],
    messages: &[Message],
    offset: usize,
) -> String {
    let mut derived = vec![];
    prepend_state_header(records, pages, messages, || offset, &mut derived);
    derived
        .iter()
        .flat_map(|m| &m.content)
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn all_complete_sources_survive_more_than_eight_pins_and_multiple_text_parts() {
    let messages: Vec<_> = (0..20)
        .map(|index| {
            message(&format!(
                "constraint {index}\n{}\nnever lose this ending",
                "🦀".repeat(200)
            ))
        })
        .collect();
    let header = render(vec![], &vec![PageKind::Constraint; 20], &messages, 500);
    for (index, source) in messages.iter().enumerate() {
        let ContentPart::Text { text } = &source.content[0] else {
            unreachable!()
        };
        assert!(header.contains(text));
        assert!(header.contains(&format!("source turn {}", 500 + index)));
    }
    let split = Message {
        role: Role::User,
        content: vec![
            ContentPart::Text {
                text: "first part".into(),
            },
            ContentPart::Text {
                text: "second part".into(),
            },
        ],
    };
    assert_eq!(
        ConstraintRecord::from_message(0, &split, PageKind::Constraint)
            .unwrap()
            .text,
        "first part\nsecond part"
    );
}

#[test]
fn later_explicit_supersession_removes_old_rule_from_active_header() {
    let old = ConstraintRecord::from_message(
        3,
        &message("use the old endpoint"),
        PageKind::SupersededConstraint { by_turn: 40 },
    )
    .unwrap();
    let header = render(
        vec![old],
        &[PageKind::Constraint],
        &[message("use the new endpoint")],
        40,
    );
    assert!(!header.contains("use the old endpoint"));
    assert!(header.contains("use the new endpoint"));
    assert!(header.contains("supersedes [3]"));
}

#[test]
fn hashes_cover_exact_text_and_source_role_does_not_become_user_authority() {
    let mut source = message("hello");
    source.role = Role::Assistant;
    let record = ConstraintRecord::from_message(3, &source, PageKind::Constraint).unwrap();
    assert_eq!(
        record.text_hash,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    let header = render(vec![record], &[], &[], 0);
    assert!(header.contains("role Assistant"));
    assert!(header.contains("not user authorization"));
}
