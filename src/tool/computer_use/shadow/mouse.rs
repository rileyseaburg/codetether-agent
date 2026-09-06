//! Mouse action planning using only shadow-held button masks.
mod transition;
use super::super::input::{ComputerUseAction as A, ComputerUseInput};
use super::{
    button, coordinates,
    types::{Geometry, Plan, Point},
};
use anyhow::{Result, ensure};
pub(super) use transition::{edge, movement};

pub(super) fn append(input: &ComputerUseInput, geometry: Geometry, plan: &mut Plan) -> Result<()> {
    let point = coordinates::resolve(input, geometry, plan.state, false)?;
    let name = if matches!(input.action, A::RightClick) {
        Some("right")
    } else {
        input.button.as_deref()
    };
    let button = button::parse(name)?;
    if matches!(input.action, A::Scroll) {
        let screen = coordinates::translate(point, geometry.client, Point::default())?;
        let delta = input.scroll_amount.unwrap_or_default() as i16 as u16;
        let wparam = usize::from(plan.state.buttons) | (usize::from(delta) << 16);
        plan.state.pointer_client = Some(point);
        plan.push(0x020a, wparam, coordinates::pack(screen)?, 0);
        return Ok(());
    }
    movement(plan, point, 0)?;
    match input.action {
        A::MouseMove => (),
        A::MouseDown => edge(plan, point, button, true, false)?,
        A::MouseUp => edge(plan, point, button, false, false)?,
        A::Click | A::RightClick | A::DoubleClick => {
            ensure!(
                plan.state.buttons & button.mask == 0,
                "Requested button is already shadow-held"
            );
            edge(plan, point, button, true, false)?;
            edge(plan, point, button, false, false)?;
            if matches!(input.action, A::DoubleClick) {
                edge(plan, point, button, true, true)?;
                edge(plan, point, button, false, false)?;
            }
        }
        _ => anyhow::bail!("Unsupported shadow mouse action"),
    }
    Ok(())
}
