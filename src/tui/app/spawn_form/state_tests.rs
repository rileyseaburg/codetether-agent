//! Tests for spawn form state navigation and argument assembly.

use super::state::{SpawnField, SpawnFormState};

#[test]
fn next_field_cycles_through_all_fields() {
    let mut form = SpawnFormState::default();
    assert_eq!(form.active, SpawnField::Name);
    form.next_field();
    assert_eq!(form.active, SpawnField::Parent);
    form.next_field();
    assert_eq!(form.active, SpawnField::Instructions);
    form.next_field();
    assert_eq!(form.active, SpawnField::Name);
}

#[test]
fn to_args_requires_name() {
    let form = SpawnFormState::default();
    assert!(form.to_args().is_none());
}

#[test]
fn to_args_builds_name_parent_instructions() {
    let mut form = SpawnFormState::default();
    form.name = "reviewer".into();
    form.parent = "build".into();
    form.instructions = "audit the PR".into();
    assert_eq!(
        form.to_args().unwrap(),
        "reviewer --parent build audit the PR"
    );
}
