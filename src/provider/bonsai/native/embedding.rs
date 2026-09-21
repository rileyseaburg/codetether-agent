//! Decode only the selected embedding row; never expand the full embedding matrix.
use super::{pq2, rotation::Rotation};
use anyhow::{Result, ensure};
use candle_core::{Device, Tensor};
pub(super) struct Embedding {
    pub bytes: Vec<u8>,
    pub width: usize,
    pub rows: usize,
    pub signs: Tensor,
}
impl Embedding {
    pub fn row(&self, id: u32, device: &Device) -> Result<Tensor> {
        ensure!(
            (id as usize) < self.rows,
            "Token ID exceeds Bonsai vocabulary"
        );
        let stride = self.width / 128 * 34;
        let offset = id as usize * stride;
        let mut values = Vec::with_capacity(self.width);
        for block in self.bytes[offset..offset + stride].chunks_exact(34) {
            values.extend(pq2::decode(block)?);
        }
        let row = Tensor::from_vec(values, (1, self.width), device)?;
        Ok(Rotation { inverse: true }.apply(&row, &self.signs)?)
    }
}
