#[path = "zlib/bit_reader.rs"]
mod bit_reader;
#[path = "zlib/deflate.rs"]
mod deflate;
#[path = "zlib/huffman.rs"]
mod huffman;

pub(crate) fn inflate_zlib(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.len() < 6 {
        return Err("zlib inflate: input too short".into());
    }
    let cmf = input[0];
    if cmf & 0x0f != 8 {
        return Err("zlib inflate: unsupported compression method".into());
    }
    if input[1] & 0x20 != 0 {
        return Err("zlib inflate: preset dictionaries are not supported".into());
    }
    deflate::inflate(&input[2..input.len() - 4])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inflate_zlib_reads_stored_blocks() {
        let input = [
            0x78, 0x01, 0x01, 0x05, 0x00, 0xfa, 0xff, b'h', b'e', b'l', b'l', b'o', 0x06, 0x2c,
            0x02, 0x15,
        ];

        assert_eq!(inflate_zlib(&input).unwrap(), b"hello");
    }
}
