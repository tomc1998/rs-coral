
mod layout;
mod layout_system;
mod children;

pub use self::layout::{LayoutComponent, LayoutStrategy};
pub use self::layout_system::layout;
pub use self::children::ChildrenComponent;

use specs;

pub type Entity = specs::Entity;

