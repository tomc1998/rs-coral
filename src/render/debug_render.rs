use utils;
use PaintController;
use entity::Entity;
use common::Color;

/// Perform a debug render to test layout. Children will be drawn on top of parents.
pub fn debug_render(c: &PaintController, root: &Entity) {
    let num_components = utils::count_component_tree(root);
    let mut visit_list = Vec::new();
    visit_list.push((root.get_offset(), root));

    // Keep track of the colour to use & how many component's we've visited
    let mut color = Color::new(1.0, 0.0, 0.0, 1.0);
    let mut num_visited : usize = 0;

    while !visit_list.is_empty() {
        // Get the node + add its children
        let (pos, node) = visit_list.remove(0);
        for c in node.get_children() { visit_list.push((pos + c.get_offset(), c)); }

        // Now draw a rect
        let size = node.get_size();
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
