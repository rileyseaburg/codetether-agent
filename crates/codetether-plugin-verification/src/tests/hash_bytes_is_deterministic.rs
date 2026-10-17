//! tests/hash bytes is deterministic for verified plugin registration.
use super::*;

#[test]
fn hash_bytes_is_deterministic() {
    let a = hash_bytes(b"hello world");
    let b = hash_bytes(b"hello world");
    assert_eq!(a, b);
    assert_ne!(a, hash_bytes(b"hello worl"));
}
