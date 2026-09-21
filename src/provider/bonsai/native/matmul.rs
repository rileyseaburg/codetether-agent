//! Candle custom operation over packed PQ2_0 bytes, without an FP16 weight copy.
use candle_core::{Layout, Result, Shape, Tensor};
pub(super) struct Pq2 {
    pub rows: usize,
    pub columns: usize,
}
impl Pq2 {
    pub fn apply(&self, input: &Tensor, packed: &Tensor) -> Result<Tensor> {
        let shape = input.dims();
        let expected = self
            .rows
            .checked_mul(self.columns / 128)
            .and_then(|n| n.checked_mul(34));
        if shape.last().copied() != Some(self.columns) || Some(packed.elem_count()) != expected {
            candle_core::bail!("PQ2_0 matrix dimensions do not match");
        }
        input
            .contiguous()?
            .apply_op2_no_bwd(&packed.contiguous()?, self)
    }
    pub fn output(&self, layout: &Layout, packed: &Layout) -> Result<Shape> {
        if self.columns == 0
            || self.columns % 128 != 0
            || self.rows == 0
            || layout.start_offset() != 0
            || packed.start_offset() != 0
            || !layout.is_contiguous()
            || !packed.is_contiguous()
        {
            candle_core::bail!("PQ2_0 requires nonempty contiguous zero-offset tensors");
        }
        if self
            .rows
            .checked_mul(self.columns / 128)
            .and_then(|n| n.checked_mul(34))
            != Some(packed.shape().elem_count())
            || layout.shape().elem_count() == 0
        {
            candle_core::bail!("PQ2_0 storage geometry mismatch");
        }
        let mut shape = layout.dims().to_vec();
        if shape.last().copied() != Some(self.columns) {
            candle_core::bail!("PQ2_0 activation width mismatch");
        }
        *shape.last_mut().unwrap() = self.rows;
        Ok(Shape::from(shape))
    }
}
