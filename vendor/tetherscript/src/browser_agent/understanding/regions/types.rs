//! Page region types.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageRegion {
    Header,
    Navigation,
    Main,
    Footer,
    Sidebar,
    Modal,
    Dialog,
    Banner,
    Complementary,
}

#[derive(Clone, Debug)]
pub struct RegionInfo {
    pub kind: PageRegion,
    pub selector: String,
    pub confidence: f64,
}

impl RegionInfo {
    pub fn from_match(selector: String, hit: (PageRegion, f64)) -> Option<Self> {
        Some(Self {
            kind: hit.0,
            selector,
            confidence: hit.1,
        })
    }
}
