use super::Entry;

pub(super) fn entries(lengths: &[u8], max_len: u8) -> Vec<Entry> {
    let mut counts = vec![0u16; max_len as usize + 1];
    for len in lengths.iter().copied().filter(|len| *len != 0) {
        counts[len as usize] += 1;
    }
    let mut next = next_codes(&counts, max_len);
    lengths
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, len)| *len != 0)
        .map(|(symbol, len)| {
            let code = next[len as usize];
            next[len as usize] += 1;
            Entry {
                symbol: symbol as u16,
                code: reverse(code, len),
                len,
            }
        })
        .collect()
}

fn next_codes(counts: &[u16], max_len: u8) -> Vec<u16> {
    let mut code = 0u16;
    let mut next = vec![0u16; max_len as usize + 1];
    for bits in 1..=max_len as usize {
        code = (code + counts[bits - 1]) << 1;
        next[bits] = code;
    }
    next
}

fn reverse(mut code: u16, len: u8) -> u16 {
    let mut out = 0;
    for _ in 0..len {
        out = (out << 1) | (code & 1);
        code >>= 1;
    }
    out
}
