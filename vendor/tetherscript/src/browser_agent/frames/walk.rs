use super::attrs::{frame_name, frame_url};
use super::path_id::frame_id_for_path;
use super::{FrameId, FrameTree};
use crate::browser::{Element, Node};

pub(super) fn collect_frames(
    nodes: &[Node],
    parent: FrameId,
    path: &mut Vec<usize>,
    tree: &mut FrameTree,
) {
    for (index, node) in nodes.iter().enumerate() {
        path.push(index);
        if let Node::Element(element) = node {
            collect_element(element, parent, path, tree);
        }
        path.pop();
    }
}

fn collect_element(
    element: &Element,
    parent: FrameId,
    path: &mut Vec<usize>,
    tree: &mut FrameTree,
) {
    if element.tag.eq_ignore_ascii_case("iframe") {
        let id = frame_id_for_path(path);
        let _ = tree.add_child_with_id(parent, id, frame_url(element), frame_name(element));
        collect_frames(&element.children, id, path, tree);
    } else {
        collect_frames(&element.children, parent, path, tree);
    }
}
