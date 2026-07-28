use super::*;

pub(super) fn upgrade_existing(
    root: &Rc<RefCell<Node>>,
    name: &str,
    definition: &registry::CustomDefinition,
) -> Result<(), String> {
    let paths = matching_paths(&root.borrow(), name, Vec::new());
    for path in paths {
        let handle = DomHandle {
            root: root.clone(),
            path,
        };
        if let Some(callback) = util::callback(&definition.value, "constructor") {
            js::call_function_with_this(callback, node_object(handle.clone()), &[])?;
        }
        if let Some(callback) = util::callback(&definition.value, "connectedCallback") {
            js::call_function_with_this(callback, node_object(handle), &[])?;
        }
    }
    Ok(())
}

fn matching_paths(node: &Node, name: &str, path: Vec<usize>) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let Node::Element(el) = node else {
        return out;
    };
    if el.tag == name {
        out.push(path.clone());
    }
    for (index, child) in el.children.iter().enumerate() {
        let mut child_path = path.clone();
        child_path.push(index);
        out.extend(matching_paths(child, name, child_path));
    }
    out
}
