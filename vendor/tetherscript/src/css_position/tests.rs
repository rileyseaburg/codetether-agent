use super::*;

fn el(position: PositionType) -> PositionedElement<usize> {
    PositionedElement {
        element: 0,
        parent: None,
        position,
        offsets: Edges::default(),
        z_index: None,
        normal_x: 10.0,
        normal_y: 20.0,
        width: 100.0,
        height: 50.0,
        computed_x: 10.0,
        computed_y: 20.0,
        margin: BoxEdges::default(),
        padding: BoxEdges::default(),
        border: BoxEdges::default(),
    }
}

mod layout;
mod position;
mod z_index;
