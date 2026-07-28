use super::Huffman;

pub(super) fn trees() -> Result<(Huffman, Huffman), String> {
    let mut lit_lengths = vec![0u8; 288];
    for len in lit_lengths.iter_mut().take(144) {
        *len = 8;
    }
    for len in lit_lengths.iter_mut().take(256).skip(144) {
        *len = 9;
    }
    for len in lit_lengths.iter_mut().take(280).skip(256) {
        *len = 7;
    }
    for len in lit_lengths.iter_mut().take(288).skip(280) {
        *len = 8;
    }
    Ok((Huffman::new(&lit_lengths)?, Huffman::new(&[5u8; 32])?))
}
