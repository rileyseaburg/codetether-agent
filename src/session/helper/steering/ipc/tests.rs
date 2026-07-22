use super::{client, endpoint::Endpoint};
use crate::session::helper::steering::{SteeringInput, clear, queue};

#[tokio::test]
async fn routes_text_to_the_active_session_owner() {
    let temp = tempfile::tempdir().unwrap();
    let session_id = "session-1234";
    let path = temp.path().join("active.sock");
    queue::open(session_id);
    let _endpoint = Endpoint::start_at(session_id, path.clone()).unwrap();

    assert!(client::send_to(&path, "change direction").await.unwrap());
    let inputs: Vec<SteeringInput> = queue::take(session_id);
    let (_, text) = inputs.into_iter().next().unwrap().into_message();

    assert_eq!(text, "change direction");
    clear(session_id);
}

#[test]
fn endpoint_path_is_stable_for_a_session() {
    let temp = tempfile::tempdir().unwrap();
    let first = super::path::for_session_in(temp.path(), "session-1234").unwrap();
    let second = super::path::for_session_in(temp.path(), "session-1234").unwrap();
    assert_eq!(first, second);
}
