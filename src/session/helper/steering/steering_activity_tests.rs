use crate::session::Session;

use super::{SteeringInput, clear, drain_into, open, push};

#[tokio::test]
async fn consumed_input_removes_its_wait_activity() {
    let mut session = Session::new().await.expect("session");
    open(&session.id);
    assert!(push(
        &session.id,
        SteeringInput::new("first".into(), vec![])
    ));
    assert!(push(
        &session.id,
        SteeringInput::new("second".into(), vec![])
    ));
    assert_eq!(drain_into(&mut session), 2);
    assert!(
        crate::tool::agent::collaboration_runtime::parent_activity::until(
            &session.id,
            tokio::time::Instant::now(),
        )
        .await
        .is_none()
    );
}

#[tokio::test]
async fn clearing_input_removes_its_wait_activity() {
    let session = Session::new().await.expect("session");
    open(&session.id);
    assert!(push(
        &session.id,
        SteeringInput::new("discarded".into(), vec![])
    ));
    clear(&session.id);
    assert!(
        crate::tool::agent::collaboration_runtime::parent_activity::until(
            &session.id,
            tokio::time::Instant::now(),
        )
        .await
        .is_none()
    );
}
