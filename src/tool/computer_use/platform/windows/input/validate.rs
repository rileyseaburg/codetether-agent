use crate::tool::computer_use::input::ComputerUseInput;

pub fn coords(input: &ComputerUseInput) -> anyhow::Result<(i32, i32)> {
    resolve(input, input.x, input.y, "x", "y")
}

pub fn drag_coords(input: &ComputerUseInput) -> anyhow::Result<(i32, i32, i32, i32)> {
    let (x1, y1) = coords(input)?;
    let (x2, y2) = resolve(input, input.x2, input.y2, "x2", "y2")?;
    Ok((x1, y1, x2, y2))
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

pub fn coordinate_mode(input: &ComputerUseInput) -> &'static str {
    match (input.hwnd.is_some(), input.client_area) {
        (true, true) => "window_client_relative",
        (true, false) => "window_frame_relative",
        (false, _) => "physical_screen",
    }
}

fn resolve(
    input: &ComputerUseInput,
    x: Option<f64>,
    y: Option<f64>,
    x_name: &str,
    y_name: &str,
) -> anyhow::Result<(i32, i32)> {
    let x = coord(x, x_name)?;
    let y = coord(y, y_name)?;
    let Some(hwnd) = input.hwnd else {
        return Ok((x, y));
    };
    let (left, top) = origin(hwnd, input.client_area)?;
    Ok((left + x, top + y))
}

fn origin(hwnd: i64, client_area: bool) -> anyhow::Result<(i32, i32)> {
    if client_area {
        return crate::platform::windows::computer_use::window::client_origin(hwnd);
    }
    let bounds = crate::platform::windows::computer_use::window::window_bounds(hwnd)?;
    Ok((bounds.left, bounds.top))
}
