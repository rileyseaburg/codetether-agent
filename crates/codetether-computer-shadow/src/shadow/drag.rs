//! Bounded drag interpolation; all coordinates are checked before posting.
use super::super::input::ComputerUseInput;
use super::{
    button, coordinates, mouse,
    types::{Geometry, Plan, Point},
};
use anyhow::Result;

pub(super) fn append(input: &ComputerUseInput, geometry: Geometry, plan: &mut Plan) -> Result<()> {
    let start = coordinates::resolve(input, geometry, plan.state, false)?;
    let end = coordinates::resolve(input, geometry, plan.state, true)?;
    let button = button::parse(input.button.as_deref())?;
    let steps = input.steps.unwrap_or(12);
    let duration = input.duration_ms.unwrap_or(500);
    mouse::movement(plan, start, 0)?;
    mouse::edge(plan, start, button, true, false)?;
    let mut elapsed = 0;
    for step in 1..=steps {
        let interpolate =
            |a: i32, b: i32| a + ((i64::from(b - a) * i64::from(step)) / i64::from(steps)) as i32;
        let point = Point {
            x: interpolate(start.x, end.x),
            y: interpolate(start.y, end.y),
        };
        let target = duration * u64::from(step) / u64::from(steps);
        mouse::movement(plan, point, target - elapsed)?;
        elapsed = target;
    }
    mouse::edge(plan, end, button, false, false)
}
