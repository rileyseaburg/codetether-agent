//! History-format regression expectation.
use super::*;
#[test]
fn ordinary_text_keeps_each_callers_separator() {
    let history = [Message {
        role: Role::User,
        content: vec![
            ContentPart::Text { text: "one".into() },
            ContentPart::Text { text: "two".into() },
        ],
    }];
    assert_eq!(
        convert(&history, false, HistoryFormat::SingleReply, str::to_owned)[0]["content"],
        "one\ntwo"
    );
    assert_eq!(
        convert(
            &history,
            false,
            HistoryFormat::ExpandedReplies,
            str::to_owned
        )[0]["content"],
        "onetwo"
    );
}
