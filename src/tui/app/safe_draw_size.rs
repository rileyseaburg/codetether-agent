use ratatui::layout::{Rect, Size};

const MAX_RENDER_CELLS: u32 = 500_000;
const MAX_RENDER_DIMENSION: u16 = 1_000;

pub fn safe_size(size: Size) -> bool {
    let cells = size.width as u32 * size.height as u32;
    size.width > 0
        && size.height > 0
        && size.width <= MAX_RENDER_DIMENSION
        && size.height <= MAX_RENDER_DIMENSION
        && cells <= MAX_RENDER_CELLS
}

pub const fn rect_for(size: Size) -> Rect {
    Rect::new(0, 0, size.width, size.height)
}

#[cfg(test)]
mod tests {
    use super::safe_size;
    use ratatui::layout::Size;

    #[test]
    fn rejects_absurd_terminal_sizes() {
        assert!(safe_size(Size::new(120, 40)));
        assert!(!safe_size(Size::new(0, 40)));
        assert!(!safe_size(Size::new(u16::MAX, u16::MAX)));
    }
}
