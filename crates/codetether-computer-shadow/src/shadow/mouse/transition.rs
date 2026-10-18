//! Logical pointer/button state transitions captured per queued message.
use super::super::{
    button::Button,
    coordinates,
    types::{Plan, Point},
};
use anyhow::{Result, ensure};

pub(in super::super) fn movement(plan: &mut Plan, point: Point, delay: u64) -> Result<()> {
    plan.state.pointer_client = Some(point);
    plan.push(
        0x0200,
        usize::from(plan.state.buttons),
        coordinates::pack(point)?,
        delay,
    );
    Ok(())
}
pub(in super::super) fn edge(
    plan: &mut Plan,
    point: Point,
    button: Button,
    down: bool,
    double: bool,
) -> Result<()> {
    let held = plan.state.buttons & button.mask != 0;
    ensure!(
        held != down,
        "Cannot duplicate down or release a button not shadow-held"
    );
    let message = if down {
        plan.state.buttons |= button.mask;
        if double { button.double } else { button.down }
    } else {
        plan.state.buttons &= !button.mask;
        button.up
    };
    plan.state.pointer_client = Some(point);
    plan.push(
        message,
        usize::from(plan.state.buttons),
        coordinates::pack(point)?,
        0,
    );
    Ok(())
}
