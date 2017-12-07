use utils;
use PaintController;
use entity::{Entity, LayoutComponent};
use common::Color;
use specs;

/// Perform a debug render to test layout. Children will be drawn on top of parents.
pub fn debug_render(c: &PaintController, root: Entity, world: &specs::World) {
    let num_components = utils::count_component_tree(root, world);
    let mut visit_list = Vec::new();

    let layout_storage = world.read();

    let layout : &LayoutComponent = layout_storage.get(root)
        .expect("Tried to layout an entity without a layout component!");

    visit_list.push((layout.offset, root));

    // Keep track of the colour to use & how many component's we've visited
    let mut color = Color::new(1.0, 0.0, 0.0, 1.0);
    let mut num_visited : usize = 0;

    while !visit_list.is_empty() {
        // Get the node + add its children
        let (pos, node) = visit_list.remove(0);
        let layout : &LayoutComponent = layout_storage.get(root)
            .expect("Tried to layout an entity without a layout component!");
        for c in utils::get_entity_children(node, world) { visit_list.push((pos + layout.offset, c)); }

        // Now draw a rect
        let size = layout.size;
        c.rect(&[pos.x as f32, pos.y as f32, size.x as f32, size.y as f32], 
               &[color.r, color.g, color.b, color.a]);

        // Increment num_visited and use this to calculate the new colour
        num_visited += 1;
        let t = num_visited as f32 / num_components as f32;
        color.r = (-2.0*t).min(1.0).max(0.0);
        color.g = if t > 0.5 {
            (-2.0*(t-0.5)).min(1.0).max(0.0)
        } else {
            (2.0*t).min(1.0).max(0.0)
        };
        color.b = (2.0*(t-0.5)).min(1.0).max(0.0);
    }
}
