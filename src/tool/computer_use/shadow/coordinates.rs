//! Checked Win32 coordinate conversion and signed 16-bit LPARAM packing.
use super::super::input::ComputerUseInput;
use super::types::{Geometry, Point, State};
use anyhow::{Result, anyhow};

pub(super) fn pack(point: Point) -> Result<isize> {
    let x = i16::try_from(point.x).map_err(|_| anyhow!("Mouse x exceeds signed 16-bit range"))?;
    let y = i16::try_from(point.y).map_err(|_| anyhow!("Mouse y exceeds signed 16-bit range"))?;
    Ok(((x as u16 as u32) | ((y as u16 as u32) << 16)) as isize)
}
pub(super) fn translate(point: Point, from: Point, to: Point) -> Result<Point> {
    let x = i64::from(point.x) + i64::from(from.x) - i64::from(to.x);
    let y = i64::from(point.y) + i64::from(from.y) - i64::from(to.y);
    Ok(Point {
        x: x.try_into()?,
        y: y.try_into()?,
    })
}
pub(super) fn resolve(
    input: &ComputerUseInput,
    geometry: Geometry,
    state: State,
    end: bool,
) -> Result<Point> {
    let (x, y) = if end {
        (input.x2, input.y2)
    } else {
        (input.x, input.y)
    };
    let point = match (x, y) {
        (Some(x), Some(y)) => {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            if input.client_area {
                point
            } else {
                translate(point, geometry.outer, geometry.client)?
            }
        }
        (None, None) if !end => state
            .pointer_client
            .ok_or_else(|| anyhow!("Supply x/y; no shadow logical pointer exists for this HWND"))?,
        _ => return Err(anyhow!("Both coordinate components are required")),
    };
    pack(point)?;
    Ok(point)
}
