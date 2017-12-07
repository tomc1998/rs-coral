//! This module defines components and types relating to layout. For the layout system, the code
//! which is in charge of performing actual layouts, see the layout_system module.

use specs;
use common::ScreenVec;

/// A layout strategy - when laying out, how should this entity position its children and size
/// itself according to the input constraints?
/// 
/// If the given entity has the wrong amount of children when laying out, then an assertion will be
/// thrown (when in debug mode).
#[derive(Clone, PartialEq, PartialOrd)]
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

}

impl LayoutStrategy {
    pub fn expected_children(&self) -> usize {
        match *self {
            LayoutStrategy::Center => 1,
            LayoutStrategy::Max => 0,
            LayoutStrategy::Proportion(_,_) => 0,
        }
    }
}

/// A component defining a component's layout - the layout strategy, the current offset, and its
/// current size.
#[derive(Clone, PartialEq)]
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

