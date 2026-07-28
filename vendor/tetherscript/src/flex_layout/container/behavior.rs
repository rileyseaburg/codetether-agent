//! Flex container behavior.

use super::{
    AlignContent, AlignItems, FlexContainer, FlexDirection, FlexWrap, Gap, JustifyContent,
};

impl FlexContainer {
    pub fn is_row(&self) -> bool {
        matches!(
            self.direction,
            FlexDirection::Row | FlexDirection::RowReverse
        )
    }

    pub fn is_reverse(&self) -> bool {
        matches!(
            self.direction,
            FlexDirection::RowReverse | FlexDirection::ColumnReverse
        )
    }
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Stretch,
            gap: Gap::default(),
        }
    }
}
