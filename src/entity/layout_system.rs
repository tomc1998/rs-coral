//! This module contains the actual layout system, which operates on layout components to size
//! them. For definitions of types relating to this see the layout module.
//! 
//! Note this module doesn't define an official 'system' known through the ECS model, as layout
//! needs to be traversed as a tree. 

use entity::*;
use common::{Constraints, ScreenVec};
use specs;

/// Layout a given entity tree starting at a root entity.
/// # Returns
/// The size of the layed out entity.
pub fn layout(root: Entity, 
              constraints: Constraints, 
              layout_storage: &mut specs::WriteStorage<LayoutComponent>,
              children_storage: &specs::ReadStorage<ChildrenComponent>
              ) -> ScreenVec {
    // Get root's layout component and children
    let root_layout = layout_storage.get(root)
        .cloned()
        .expect("Tried to layout an entity without a layout component!");
    let children = children_storage.get(root)
        .cloned()
        .expect("Tried to layout an entity without a children component!").children;

    // Make sure we have the right amount of children for this layout strategy
    debug_assert_eq!(children.len(), 
                     root_layout.strategy.expected_children(), 
                     "Center layout strategy must have 1 child only");

    let final_size;
    match root_layout.strategy {
        LayoutStrategy::Center => {
            // Must be 1 child
            let child = children[0];
            let child_size = layout(child, constraints, layout_storage, children_storage);
            let child_layout = layout_storage.get_mut(child)
                .expect("Tried to layout an entity without a layout component!");
            child_layout.offset.x = root_layout.size.x / 2 - child_size.x / 2;
            child_layout.offset.y = root_layout.size.y / 2 - child_size.y / 2;
            final_size = ScreenVec::new(constraints.max_w as i32, constraints.max_h as i32);
        }
        LayoutStrategy::Max => {
            final_size = ScreenVec::new(constraints.max_w as i32, constraints.max_h as i32);
        }
    }

    let root_layout = layout_storage.get_mut(root).unwrap();
    root_layout.size = final_size;

    return final_size;
}

