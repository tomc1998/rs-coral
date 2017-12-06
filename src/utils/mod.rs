use Component;

/// A utility to count the number of components given a root node (including the root).
pub fn count_component_tree(root: &Component) -> usize {
    let mut visit_list = Vec::new();
    let mut count = 0;
    visit_list.push(root);
    while !visit_list.is_empty() {
        count += 1;
        let node = visit_list.remove(0);
        for c in node.get_children() { visit_list.push(c); }
    }
    return count;
}
