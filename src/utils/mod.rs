use specs::*;
use entity::{ChildrenComponent, Entity};

pub fn get_entity_children<'a>(e: Entity, 
                               children_storage: &'a ReadStorage<ChildrenComponent>) -> Vec<Entity> {
    children_storage.get(e).map(|c| c.children.clone()).unwrap_or(Vec::new())
}

/// A utility to count the number of components given a root node (including the root).
pub fn count_component_tree(root: Entity, 
                            children_storage: &ReadStorage<ChildrenComponent>) -> usize {
    let mut visit_list = Vec::new();
    let mut count = 0;
    visit_list.push(root);
    while !visit_list.is_empty() {
        count += 1;
        let node = visit_list.remove(0);
        for c in get_entity_children(node, children_storage) {
            visit_list.push(c); 
        }
    }
    return count;
}
