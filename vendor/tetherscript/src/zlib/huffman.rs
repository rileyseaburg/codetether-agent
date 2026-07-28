use super::bit_reader::BitReader;

#[path = "huffman/build.rs"]
mod build;

struct Entry {
    symbol: u16,
    code: u16,
    len: u8,
}

pub(super) struct Huffman {
    entries: Vec<Entry>,
    max_len: u8,
}

impl Huffman {
    pub(super) fn new(lengths: &[u8]) -> Result<Self, String> {
        let max_len = lengths.iter().copied().max().unwrap_or(0);
        Ok(Self {
            entries: build::entries(lengths, max_len),
            max_len,
        })
    }

    pub(super) fn decode(&self, reader: &mut BitReader<'_>) -> Result<u16, String> {
        let mut code = 0u16;
        for len in 1..=self.max_len {
            code |= (reader.read_bits(1)? as u16) << (len - 1);
            if let Some(entry) = self
                .entries
                .iter()
                .find(|entry| entry.len == len && entry.code == code)
            {
                return Ok(entry.symbol);
            }
        }
        Err("zlib inflate: invalid huffman code".into())
    }
}
