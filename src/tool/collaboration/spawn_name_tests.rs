use super::{generated, resolve};

#[test]
fn generated_names_are_canonical_path_segments() {
    assert!(resolve(Some(generated())).is_ok());
}

#[test]
fn rejects_noncanonical_or_reserved_names() {
    for name in ["root", "bad-name", "Bad", "has/slash", ""] {
        assert!(resolve(Some(name.into())).is_err(), "accepted {name}");
    }
}
