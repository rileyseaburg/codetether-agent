//! Pure request-to-message planning, independent of the host OS.
use super::super::input::{ComputerUseAction as A, ComputerUseInput};
use super::{
    drag, keyboard, mouse, stop,
    types::{Geometry, Plan, State},
    validate,
};
use anyhow::Result;

pub(super) fn build(input: &ComputerUseInput, geometry: Geometry, state: State) -> Result<Plan> {
    validate::request(input)?;
    let mut plan = Plan {
        state,
        ..Plan::default()
    };
    match input.action {
        A::Status => (),
        A::Stop => stop::append(&mut plan)?,
        A::TypeText | A::PressKey => keyboard::append(input, &mut plan)?,
        A::Drag => drag::append(input, geometry, &mut plan)?,
        A::Click
        | A::RightClick
        | A::DoubleClick
        | A::MouseMove
        | A::MouseDown
        | A::MouseUp
        | A::Scroll => mouse::append(input, geometry, &mut plan)?,
        _ => anyhow::bail!("Unsupported shadow action; no physical fallback"),
    }
    Ok(plan)
}
