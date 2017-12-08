extern crate coral;

use coral::entity::LayoutStrategy;
use coral::common::ScreenVec;

fn setup_entities(coral: &mut coral::Coral) {
    let root = coral.create_entity(None, LayoutStrategy::Center);

    // Create custom layout for this entity which mimics a LayoutStrategy::Proportion(0.5, 0.5).
    coral.create_entity(
        Some(root), 
        LayoutStrategy::Custom(
            |root, c, layout_storage, _, _| -> ScreenVec {
                let root_layout = layout_storage.get_mut(root).unwrap();
                let final_size = ScreenVec::new(c.max_w as i32/2, 
                                                c.max_h as i32/2);
                root_layout.size = final_size;
                return final_size;
            }));

    coral.set_root(root);
}

fn main() {
    let mut coral = coral::Coral::new();
    coral.config.debug_drawing = true;
    setup_entities(&mut coral);
    coral.start();
}

