use crate::Error;

/// Validated physical-pixel rectangle with exclusive right/bottom edges.
/// Negative origins are supported; area is limited to 34 million pixels.
///
/// ```
/// use codetether_companion_desktop::Bounds;
/// assert_eq!(Bounds::new(-100, 0, 100, 50).unwrap().width(), 200);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bounds {
    edges: [i32; 4],
}
impl Bounds {
    /// Construct from desktop coordinates (left, top, right, bottom).
    /// Returns validated bounds, or [`Error::Geometry`] for invalid dimensions.
    /// See the type example for construction.
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Result<Self, Error> {
        let width = i64::from(right) - i64::from(left);
        let height = i64::from(bottom) - i64::from(top);
        if width <= 0 || height <= 0 || width > 34_000_000 / height {
            return Err(Error::Geometry);
        }
        Ok(Self {
            edges: [left, top, right, bottom],
        })
    }
    /// Rectangle edges in physical pixels: left, top, right, bottom.
    pub fn edges(self) -> [i32; 4] {
        self.edges
    }
    /// Width in physical pixels.
    pub fn width(self) -> u32 {
        (i64::from(self.edges[2]) - i64::from(self.edges[0])) as u32
    }
    /// Height in physical pixels.
    pub fn height(self) -> u32 {
        (i64::from(self.edges[3]) - i64::from(self.edges[1])) as u32
    }
    /// Whether a physical desktop point lies inside this monitor only.
    pub fn contains(self, x: i32, y: i32) -> bool {
        x >= self.edges[0] && x < self.edges[2] && y >= self.edges[1] && y < self.edges[3]
    }
}
