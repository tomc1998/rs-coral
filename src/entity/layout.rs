//! This module defines components and types relating to layout. For the layout system, the code
//! which is in charge of performing actual layouts, see the layout_system module.

use specs;
use common::ScreenVec;

/// A layout strategy - when laying out, how should this entity position its children and size
/// itself according to the input constraints?
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum LayoutStrategy {
    Center,
}

/// A component defining a component's layout - the layout strategy, the current offset, and its
/// current size.
#[derive(Clone, Hash, Eq, PartialEq)]
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

