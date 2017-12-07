//! This module contains the actual layout system, which operates on layout components to size
//! them. For definitions of types relating to this see the layout module.
//! 
//! Note this module doesn't define an official 'system' known through the ECS model, as layout
//! needs to be traversed as a tree. 

use entity::*;
use common;
use specs;

/// Layout a given entity tree starting at a root entity.
pub fn layout(root: Entity, constraints: common::Constraints, world: &specs::World) {
    debug_assert!(world.is_alive(root));
    // Get root's layout component and children
    let layout_storage = world.write_with_id(root.id() as usize);
    let layout : &LayoutComponent = layout_storage.get(root)
        .expect("Tried to layout an entity without a layout component!");
    let children_storage = world.write_with_id(root.id() as usize);
    let children : &ChildrenComponent = children_storage.get(root)
        .expect("Tried to layout an entity without a children component!");
}

