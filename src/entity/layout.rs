//! This module defines components and types relating to layout. For the layout system, the code
//! which is in charge of performing actual layouts, see the layout_system module.

use specs;
use common::{ScreenVec, Constraints};
use entity::*;

/// A layout strategy - when laying out, how should this entity position its children and size
/// itself according to the input constraints?
/// 
/// If the given entity has the wrong amount of children when laying out, then an assertion will be
/// thrown (when in debug mode).
#[derive(Copy)]
pub enum LayoutStrategy {

    /// Num children: 1
    /// 
    /// Center the given child. Child can expand to fill the max constraints of the parent.
    Center,

    /// Num children: 0
    /// 
    /// Maximise this component's size according to the constraints.
    Max,

    /// Num children: 0
    /// 
    /// Size this component to be a lerp between the constraint's min / max. 
    /// Works separately in x / y.
    Proportion(f32, f32),

    /// Num children: Any
    /// 
    /// Implement a custom layout function for this component and all of its children.
    /// Pass a function pointer which takes an entity ID and constraints, plus a reference to
    /// layout + children storage, then lays out the given entity + all of its children.
    /// 
    /// Also takes a mutable reference to the LayoutSystem, to call back into layout().
    /// 
    /// # Example
    /// ```
    /// fn my_layout(&mut self, root: Entity, 
    ///           c: Constraints, 
    ///           layout_storage: &mut specs::WriteStorage<LayoutComponent>,
    ///           children_storage: &specs::ReadStorage<ChildrenComponent>,
    ///           layout_system: &mut LayoutSystem) -> ScreenVec {
    ///           
    ///     // Get root's layout component and children using the layout and children storage:
    ///     let root_layout = layout_storage.get(root)
    ///         .cloned()
    ///         .expect("Tried to layout an entity without a layout component!");
    ///     let children = children_storage.get(root)
    ///         .cloned()
    ///         .expect("Tried to layout an entity without a children component!").children;
    ///         
    ///     // First call all the children's layouts (realistically you'd change the constraints
    ///     // for the children)
    ///     for child in children {
    ///         layout_system.layout(child, c, layout_storage, children_storage);
    ///     }
    ///     
    ///     // Now layout this entity (notice the lack of .cloned() here, this is a mutable
    ///     // reference rather than just a value)
    ///     let root_layout = layout_storage.get_mut(root).unwrap();
    ///     let final_size = ScreenVec::new(c.max_w, c.max_h); // As big as possible
    ///     root_layout.size = final_size;
    ///     
    ///     // Finally, return the final size of this component.
    ///     return final_size;
    /// }
    /// ```
    Custom(fn(Entity, Constraints, 
            &mut specs::WriteStorage<LayoutComponent>, 
            &specs::ReadStorage<ChildrenComponent>,
            &mut LayoutSystem) -> ScreenVec), 
}

impl Clone for LayoutStrategy {
    fn clone(&self) -> Self { 
        *self 
    }
} 

impl LayoutStrategy {
    /// Returns the number of children expected, or none if the amount of children is variable.
    pub fn expected_children(&self) -> Option<usize> {
        match *self {
            LayoutStrategy::Center => Some(1),
            LayoutStrategy::Max => Some(0),
            LayoutStrategy::Proportion(_,_) => Some(0),
            LayoutStrategy::Custom(_) => None,
        }
    }
}

/// A component defining a component's layout - the layout strategy, the current offset, and its
/// current size.
#[derive(Clone)]
pub struct LayoutComponent {
    pub offset: ScreenVec,
    pub size: ScreenVec,
    pub strategy: LayoutStrategy,
    pub invalidated: bool,
}

impl LayoutComponent {
    pub fn new(strategy: LayoutStrategy) -> LayoutComponent {
        LayoutComponent {
            offset: ScreenVec::new(0,0),
            size: ScreenVec::new(0,0),
            strategy: strategy,
            invalidated: true,
        }
    }
}

impl specs::Component for LayoutComponent {
    type Storage = specs::VecStorage<Self>;
}

