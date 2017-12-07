extern crate coral;

use coral::entity::LayoutStrategy;

fn setup_entities(coral: &mut coral::Coral) {
    let root = coral.create_entity(None, LayoutStrategy::Center);
    coral.create_entity(Some(root), LayoutStrategy::Proportion(0.5, 0.5));

    coral.set_root(root);
}

fn main() {
    let mut coral = coral::Coral::new();
    coral.config.debug_drawing = true;
    setup_entities(&mut coral);
    coral.start();
}
