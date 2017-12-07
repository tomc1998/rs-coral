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
              c: Constraints, 
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
            let child_size = layout(child, 
                                    Constraints::new(0, 0, c.max_w, c.max_h), 
                                    layout_storage, 
                                    children_storage);
            let child_layout = layout_storage.get_mut(child)
                .expect("Tried to layout an entity without a layout component!");
            child_layout.offset.x = c.max_w as i32 / 2 - child_size.x / 2;
            child_layout.offset.y = c.max_h as i32 / 2 - child_size.y / 2;
            final_size = ScreenVec::new(c.max_w as i32, c.max_h as i32);
        }
        LayoutStrategy::Max => {
            final_size = ScreenVec::new(c.max_w as i32, c.max_h as i32);
        }
        LayoutStrategy::Proportion(x, y) => {
            // We need the min / max in case of weird FP error
            final_size = ScreenVec::new(
                (c.min_w as i32+ 
                 ((c.max_w - c.min_w) as f32 * x) as i32)
                .max(c.min_w as i32)
                .min(c.max_w as i32), 
                (c.min_h as i32 + 
                 ((c.max_h - c.min_h) as f32 * y) as i32)
                .max(c.min_h as i32)
                .min(c.max_h as i32));
        }
    }

    let root_layout = layout_storage.get_mut(root).unwrap();
    info!("Id: {} | Size: {}, {}", root.id(), final_size.x, final_size.y);
    root_layout.size = final_size;

    return final_size;
}

