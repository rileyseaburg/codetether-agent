use super::*;

pub(super) struct Hit {
    pub path: Vec<usize>,
    z: i64,
    order: usize,
}

pub(super) fn hits(root: &Rc<RefCell<Node>>, x: i64, y: i64) -> Vec<Hit> {
    let document = root_to_document(root);
    let css = LAYOUT_CSS.with(|css| css.borrow().clone());
    let layout = browser::layout_document(&document, &css, constants::DEFAULT_VIEWPORT_WIDTH);
    let mut hits = all_by_selector(root, "*")
        .into_iter()
        .enumerate()
        .filter_map(|(order, path)| {
            let layout_box = browser::find_layout_box_at_path(&layout, &path)?;
            hit_style::matches(layout_box, x, y).then_some(Hit {
                path,
                z: hit_style::z_index(layout_box),
                order,
            })
        })
        .collect::<Vec<_>>();
    hits.sort_by(|left, right| right.z.cmp(&left.z).then(right.order.cmp(&left.order)));
    hits
}
