//! Priority ordering for resource scheduling.
use core::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcePriority {
    Css,
    HeadScript,
    Image,
    Media,
    Frame,
    AsyncScript,
    Preload,
    Prefetch,
}

impl ResourcePriority {
    pub fn weight(self) -> u8 {
        match self {
            Self::Css => 0,
            Self::HeadScript => 1,
            Self::Preload => 2,
            Self::Image => 3,
            Self::Media => 4,
            Self::Frame => 5,
            Self::AsyncScript => 6,
            Self::Prefetch => 7,
        }
    }
}

impl Ord for ResourcePriority {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight().cmp(&self.weight())
    }
}
impl PartialOrd for ResourcePriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Stylesheet,
    Script,
    Image,
    Video,
    Audio,
    Source,
    Iframe,
    Preload,
    Prefetch,
    DnsPrefetch,
    Preconnect,
    Other,
}
