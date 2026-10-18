//! Release only messages held by this backend; never touch physical input.
use super::{button, keyboard, mouse, types::Plan};
use anyhow::{Result, anyhow};

pub(super) fn append(plan: &mut Plan) -> Result<()> {
    for name in ["left", "right", "middle"] {
        let button = button::parse(Some(name))?;
        if plan.state.buttons & button.mask != 0 {
            let point = plan
                .state
                .pointer_client
                .ok_or_else(|| anyhow!("Held shadow button has no logical coordinate"))?;
            mouse::edge(plan, point, button, false, false)?;
        }
    }
    keyboard::release(plan);
    // Pointer state is cleared by runtime only after all releases queue.
    Ok(())
}
