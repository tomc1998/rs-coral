use utils;
use PaintController;
use entity::{Entity, LayoutComponent, ChildrenComponent};
use common::Color;
use specs::*;

#[derive(Clone)]
pub struct DebugRenderSystem<'a> {
    /// The paint controller to use
    pub c: Option<PaintController<'a>>,
    /// The root entity to render
    pub root: Option<Entity>,
}

impl<'a> DebugRenderSystem<'a> {
    pub fn new() -> DebugRenderSystem<'a> {
        DebugRenderSystem {
            c: None,
            root: None,
        }
    }

    fn debug_render(&self,
                    c: &PaintController, 
                    root: Entity, 
                    layout_storage: &ReadStorage<LayoutComponent>,
                    children_storage: &ReadStorage<ChildrenComponent>) {
        let num_components = utils::count_component_tree(root, children_storage);
        let mut visit_list = Vec::new();

        let layout : &LayoutComponent = layout_storage.get(root)
            .expect("Tried to layout an entity without a layout component!");

        visit_list.push((layout.offset, root));

        // Keep track of the colour to use & how many component's we've visited
        let mut color = Color::new(1.0, 0.0, 0.0, 1.0);
        let mut num_visited : usize = 0;

        while !visit_list.is_empty() {
            // Get the node + add its children
            let (pos, node) = visit_list.remove(0);
            let layout : &LayoutComponent = layout_storage.get(node)
                .expect("Tried to layout an entity without a layout component!");
            for c in utils::get_entity_children(node, children_storage) {
                let child_layout = layout_storage.get(c)
                    .expect("Tried to paint an entity without a layout component!");
                visit_list.push((pos + child_layout.offset, c)); 
            }

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
}

/// Perform a debug render to test layout. Children will be drawn on top of parents.
impl<'a, 'b> System<'a> for DebugRenderSystem<'b> {
    type SystemData = (ReadStorage<'a, LayoutComponent>, 
                       ReadStorage<'a, ChildrenComponent>);
    fn run(&mut self, (layout_storage, children_storage): Self::SystemData) {
        debug_assert!(self.c.is_some(), "Trying to paint without a valid PaintController!");
        debug_assert!(self.root.is_some(), "Trying to paint without a valid root!");
        self.debug_render(self.c.as_ref().unwrap(), self.root.unwrap(), &layout_storage, &children_storage);
    }
}
