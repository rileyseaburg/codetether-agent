//! Bounded framing and serialization tests use only in-memory streams.
use super::super::{
    codec,
    framing::{REQUEST_LIMIT, RESPONSE_LIMIT, read_frame},
};
use tokio::io::BufReader;

#[tokio::test]
async fn frame_boundaries_and_eof() {
    let mut reader = BufReader::with_capacity(2, &b"abc\nxy\n"[..]);
    assert_eq!(read_frame(&mut reader, 3).await.unwrap().unwrap(), b"abc");
    assert_eq!(read_frame(&mut reader, 3).await.unwrap().unwrap(), b"xy");
    assert!(read_frame(&mut reader, 3).await.unwrap().is_none());
    for data in [&b"abcd\n"[..], &b"abc"[..], &b"abcd"[..]] {
        assert!(read_frame(&mut BufReader::new(data), 3).await.is_err());
    }
}

#[tokio::test]
async fn production_frame_limits_are_enforced() {
    for limit in [REQUEST_LIMIT, RESPONSE_LIMIT] {
        let mut bytes = vec![b'x'; limit];
        bytes.push(b'\n');
        assert_eq!(
            read_frame(&mut BufReader::new(&bytes[..]), limit)
                .await
                .unwrap()
                .unwrap()
                .len(),
            limit
        );
        bytes.insert(limit, b'x');
        assert!(
            read_frame(&mut BufReader::new(&bytes[..]), limit)
                .await
                .is_err()
        );
    }
}

#[test]
fn serialization_limit_counts_json_escaping() {
    assert_eq!(codec::encode(&"ab", 4).unwrap(), b"\"ab\"");
    assert!(codec::encode(&"ab", 3).is_err());
    assert!(codec::encode(&"\n", 3).is_err());
}
