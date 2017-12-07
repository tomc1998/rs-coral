use specs::World;
use entity::{ChildrenComponent, Entity};

pub fn get_entity_children<'a>(e: Entity, world: &'a World) -> Vec<Entity> {
    world.read::<ChildrenComponent>().get(e)
        .cloned()
        .expect("Trying to get the children of an entity without a children component.")
        .children
}

/// A utility to count the number of components given a root node (including the root).
pub fn count_component_tree(root: Entity, world: &World) -> usize {
    let mut visit_list = Vec::new();
    let mut count = 0;
    visit_list.push(root);
    while !visit_list.is_empty() {
        count += 1;
        let node = visit_list.remove(0);
        let children_component = get_entity_children(root, world);
        for c in get_entity_children(root, world) { visit_list.push(c); }
    }
    return count;
}
