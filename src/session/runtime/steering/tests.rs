use crate::provider::ContentPart;
use crate::session::{ImageAttachment, Session};

use super::*;

#[tokio::test]
async fn drains_fifo_text_and_images_into_transcript() {
    let mut session = Session::new().await.expect("session");
    open(&session.id);
    assert!(push(
        &session.id,
        SteeringInput::new("first".into(), vec![])
    ));
    let image = ImageAttachment {
        data_url: "data:image/png;base64,AA==".into(),
        mime_type: Some("image/png".into()),
    };
    assert!(push(
        &session.id,
        SteeringInput::new("second".into(), vec![image])
    ));

    assert_eq!(drain_into(&mut session), 2);
    assert!(matches!(
        &session.messages[0].content[0],
        ContentPart::Text { text } if text == "first"
    ));
    assert!(matches!(
        &session.messages[1].content[1],
        ContentPart::Image { mime_type, .. } if mime_type.as_deref() == Some("image/png")
    ));
    clear(&session.id);
}

#[tokio::test]
async fn final_empty_drain_closes_inbox_atomically() {
    let mut session = Session::new().await.expect("session");
    open(&session.id);
    assert_eq!(drain_or_close_into(&mut session), 0);
    assert!(!push(
        &session.id,
        SteeringInput::new("late".into(), vec![])
    ));
}
