use entity;
use specs;

/// A component for entity children relationships
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ChildrenComponent {
    pub children: Vec<entity::Entity>,
}

impl ChildrenComponent {
    pub fn new() -> ChildrenComponent {
        ChildrenComponent {
            children: Vec::new()
        }
    }
}

impl specs::Component for ChildrenComponent {
    type Storage = specs::VecStorage<Self>;
}

