use super::Resolver;

#[test]
fn keeps_large_bracketed_paste_chunks_unmodified() {
    let mut resolver = Resolver::default();
    let start = b"\x1b[200~first\n".to_vec();
    let middle = b"second\nthird".to_vec();
    let end = b"\nfourth\x1b[201~".to_vec();

    assert_eq!(resolver.resolve(start.clone()), start);
    assert_eq!(resolver.resolve(middle.clone()), middle);
    assert_eq!(resolver.resolve(end.clone()), end);
}

#[test]
fn preserves_two_megabyte_bracketed_paste_across_reads() {
    let mut paste = b"\x1b[200~".to_vec();
    paste.extend(vec![b'x'; 2 * 1024 * 1024]);
    paste.extend(b"\x1b[201~");
    let mut resolver = Resolver::default();
    let actual = paste
        .chunks(64 * 1024)
        .flat_map(|chunk| resolver.resolve(chunk.to_vec()))
        .collect::<Vec<_>>();
    assert_eq!(actual, paste);
}

#[test]
fn recognizes_markers_split_across_reads() {
    let mut resolver = Resolver::default();
    let chunks = [
        b"\x1b[2".as_slice(),
        b"00~one\ntwo\x1b[20".as_slice(),
        b"1~".as_slice(),
    ];
    let actual = chunks
        .iter()
        .flat_map(|chunk| resolver.resolve(chunk.to_vec()))
        .collect::<Vec<_>>();

    assert_eq!(actual, b"\x1b[200~one\ntwo\x1b[201~");
    assert_eq!(
        resolver.resolve(b"three\nfour".to_vec()),
        b"\x1b[200~three\nfour\x1b[201~"
    );
}
