//! Same-session continuation keeps its owner while independent entry is refused.

use super::{RunGuard, execution_scope::ExecutionScope, steering};

#[tokio::test]
async fn nested_handoff_keeps_owner_and_reopens_steering() {
    let id = uuid::Uuid::new_v4().to_string();
    let mut guard = RunGuard::open(&id).unwrap();
    assert_eq!(
        steering::drain_or_close_into(&mut {
            let mut session = crate::session::Session::new().await.unwrap();
            session.id = id.clone();
            session
        }),
        0
    );
    guard
        .run(async {
            let mut scope = ExecutionScope::open(&id)?;
            scope
                .run(async {
                    assert!(RunGuard::open(&id).is_err());
                    assert!(steering::send(&id, "continue with this correction").await?);
                    Ok(())
                })
                .await
        })
        .await
        .unwrap();
    assert!(RunGuard::open(&id).is_err());
    drop(guard);
    assert!(RunGuard::open(&id).is_ok());
}

#[tokio::test]
async fn independent_entry_cannot_borrow_a_matching_session() {
    let id = uuid::Uuid::new_v4().to_string();
    let _guard = RunGuard::open(&id).unwrap();
    let result = ExecutionScope::open(&id);
    assert!(result.is_err());
}
