use crate::tool::computer_use::input::ComputerUseInput;

pub fn coords(input: &ComputerUseInput) -> anyhow::Result<(i32, i32)> {
    Ok((coord(input.x, "x")?, coord(input.y, "y")?))
}

pub fn coord(value: Option<f64>, name: &str) -> anyhow::Result<i32> {
    let value = value.ok_or_else(|| anyhow::anyhow!("{name} is required"))?;
    anyhow::ensure!(value.is_finite(), "{name} must be finite");
    anyhow::ensure!(
        value >= i32::MIN as f64 && value <= i32::MAX as f64,
        "{name} out of range"
    );
    Ok(value.round() as i32)
}
