//! Construct native Candle tensors from validated packed weights and transforms.
use super::{Index, config::Config, linear::Linear, model::Model};
use anyhow::{Result, ensure};
use candle_core::Device;
use std::io::{Read, Seek};
pub(super) fn model<R: Read + Seek>(
    index: &Index,
    reader: &mut R,
    device: &Device,
) -> Result<Model> {
    super::validate(index)?;
    ensure!(
        device.is_cuda(),
        "Native Bonsai requires the candle-cuda build; CPU fallback is disabled"
    );
    let config = Config::read(index)?;
    let signs = super::signs::load(index, device)?;
    let embedding = super::load_embedding::load(index, reader, &config, &signs)?;
    anyhow::ensure!(
        index
            .tensors
            .get("output.weight")
            .is_some_and(|t| t.shape == [config.hidden, embedding.rows]),
        "Bonsai output/embedding dimensions differ"
    );
    let output = Linear::load(index, reader, "output.weight", &signs, device)?;
    let mask = super::output_mask::load(index, device)?;
    let norm = super::norm_weight::load(index, reader, "output_norm.weight", device)?;
    let mut layers = Vec::with_capacity(config.layers);
    for layer in 0..config.layers {
        layers.push(super::load_layer::load(
            index, reader, layer, &signs, device,
        )?);
    }
    let rope = super::rope::Rope::new(&config, device)?;
    Ok(Model {
        config,
        embedding,
        output,
        norm,
        mask,
        layers,
        rope,
        device: device.clone(),
        position: 0,
    })
}
