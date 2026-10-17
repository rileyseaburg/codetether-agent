//! scan workspace for index construction.

use super::*;

pub(super) fn scan_workspace(root: &Path, options: &BuildOptions) -> IndexInputs {
    let mut data = IndexInputs::default();
    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| should_descend(entry, root, options.include_hidden));

    for entry in walker.filter_map(std::result::Result::ok) {
        scan_entry(entry, root, options, &mut data);
    }
    data
}
