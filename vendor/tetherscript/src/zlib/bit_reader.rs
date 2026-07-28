pub(super) struct BitReader<'a> {
    data: &'a [u8],
    byte: usize,
    bit: u8,
}

impl<'a> BitReader<'a> {
    pub(super) fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            byte: 0,
            bit: 0,
        }
    }

    pub(super) fn read_bits(&mut self, count: u8) -> Result<u32, String> {
        let mut value = 0;
        for offset in 0..count {
            if self.byte >= self.data.len() {
                return Err("zlib inflate: unexpected end of input".into());
            }
            let bit = (self.data[self.byte] >> self.bit) & 1;
            value |= (bit as u32) << offset;
            self.bit += 1;
            if self.bit == 8 {
                self.bit = 0;
                self.byte += 1;
            }
        }
        Ok(value)
    }

    pub(super) fn align_byte(&mut self) {
        if self.bit != 0 {
            self.bit = 0;
            self.byte += 1;
        }
    }

    pub(super) fn read_byte(&mut self) -> Result<u8, String> {
        self.align_byte();
        let byte = *self
            .data
            .get(self.byte)
            .ok_or_else(|| "zlib inflate: unexpected end of input".to_string())?;
        self.byte += 1;
        Ok(byte)
    }
}
