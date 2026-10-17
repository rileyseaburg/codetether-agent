//! Layer-scoped typed access to GGUF weights.
use super::{Index, linear::Linear};
use anyhow::Result;
use candle_core::{Device, Tensor};
use std::{
    collections::HashMap,
    io::{Read, Seek},
};
pub(super) struct Weights<'a, R> {
    pub index: &'a Index,
    pub reader: &'a mut R,
    pub signs: &'a HashMap<usize, Tensor>,
    pub device: &'a Device,
    pub prefix: String,
}
impl<R: Read + Seek> Weights<'_, R> {
    pub fn linear(&mut self, name: &str) -> Result<Linear> {
        Linear::load(
            self.index,
            self.reader,
            &format!("{}{name}.weight", self.prefix),
            self.signs,
            self.device,
        )
    }
    pub fn tensor(&mut self, name: &str) -> Result<Tensor> {
        super::norm_weight::load(
            self.index,
            self.reader,
            &format!("{}{name}", self.prefix),
            self.device,
        )
    }
}
