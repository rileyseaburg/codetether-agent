//! Tests for derived mux resume names.

use super::derive;
use crate::mux::registry::validate_name;

#[test]
fn derives_a_name_from_the_session_id_prefix() {
    assert_eq!(
        derive("bd4e55f2-627e-4fa5-9ed4-a9339f6721a6"),
        "resume-bd4e55f2"
    );
}

#[test]
fn derived_names_pass_registry_validation() {
    let name = derive("bd4e55f2-627e-4fa5-9ed4-a9339f6721a6");
    assert!(validate_name(&name).is_ok(), "{name}");
}

#[test]
fn drops_characters_outside_the_mux_alphabet() {
    let name = derive("ab.cd/ef!gh");
    assert_eq!(name, "resume-abcdefgh");
    assert!(validate_name(&name).is_ok());
}

#[test]
fn long_ids_stay_within_the_name_budget() {
    let name = derive(&"a".repeat(200));
    assert!(name.len() <= 64, "{name}");
    assert!(validate_name(&name).is_ok());
}
