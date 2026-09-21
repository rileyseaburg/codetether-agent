//! CPU reference for the model's signed normalized block Hadamard transforms.
use anyhow::{Result, ensure};
pub(super) fn transform(values: &mut [f32], signs: &[f32], inverse: bool) -> Result<()> {
    ensure!(
        values.len() == signs.len() && values.len() % 1024 == 0,
        "Invalid Bonsai Hadamard dimensions"
    );
    ensure!(
        signs.iter().all(|s| *s == -1.0 || *s == 1.0),
        "Hadamard signs must be +/-1"
    );
    if !inverse {
        for (value, sign) in values.iter_mut().zip(signs) {
            *value *= sign;
        }
    }
    for block in values.chunks_exact_mut(1024) {
        let mut stride = 1;
        while stride < 1024 {
            for group in block.chunks_exact_mut(stride * 2) {
                for i in 0..stride {
                    let a = group[i];
                    let b = group[i + stride];
                    group[i] = a + b;
                    group[i + stride] = a - b;
                }
            }
            stride *= 2;
        }
        for value in block {
            *value /= 32.0;
        }
    }
    if inverse {
        for (value, sign) in values.iter_mut().zip(signs) {
            *value *= sign;
        }
    }
    Ok(())
}
