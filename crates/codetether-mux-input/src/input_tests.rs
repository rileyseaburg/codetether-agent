use super::{INPUT_CHUNK_BYTES, chunks};

#[test]
fn two_megabyte_paste_uses_bounded_protocol_frames() {
    let data = vec![b'x'; 2 * 1024 * 1024 + 12];
    let chunks = chunks(&data).collect::<Vec<_>>();

    assert_eq!(chunks.len(), 33);
    assert!(chunks.iter().all(|chunk| chunk.len() <= INPUT_CHUNK_BYTES));
    assert_eq!(
        chunks.iter().map(|chunk| chunk.len()).sum::<usize>(),
        data.len()
    );
}

#[test]
fn empty_input_has_no_protocol_frames() {
    assert_eq!(chunks(&[]).count(), 0);
}
