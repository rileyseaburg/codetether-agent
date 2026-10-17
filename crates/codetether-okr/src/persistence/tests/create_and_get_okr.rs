//! Repository test_create_and_get_okr regression.

use super::*;

#[tokio::test]
async fn test_create_and_get_okr() {
    let repo = temp_repo();

    let mut okr = Okr::new("Test Objective", "Description");
    let kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
    okr.add_key_result(kr);

    let created = repo.create_okr(okr).await.unwrap();

    let fetched = repo.get_okr(created.id).await.unwrap();
    assert!(fetched.is_some());
    assert_eq!(fetched.unwrap().title, "Test Objective");
}
