use common::{Constraints, ScreenVec};
pub use render::Controller as PaintController;

/// Components form the basis for an application. Each component defines a function to layout
/// itself and any children it has given some layout constraints, plus a function to generate
/// vertex data from an on screen position
pub trait Component {
    /// Get this component's current on-screen size
    fn get_size(&self) -> ScreenVec;
    /// Layout this component. This should modify the size.
    fn layout(&mut self, constraints: Constraints);
    /// Paint this component with the paintcontroller, given an on-screen region that this
    /// component exists in.
    fn paint(&self, r: &PaintController, pos: ScreenVec, size: ScreenVec);
}
