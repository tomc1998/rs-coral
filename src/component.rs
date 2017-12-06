use common::{Constraints, ScreenVec};
use PaintController;

/// Components form the basis for an application. Each component defines a function to layout
/// itself and any children it has given some layout constraints, plus a function to generate
/// vertex data from an on screen position
pub trait Component {
    /// Get this component's current on-screen size
    fn get_size(&self) -> ScreenVec;
    /// Layout this component and all children given a size. This should modify the size of the
    /// component as returned by get_size.
    fn layout(&mut self, constraints: Constraints);
    /// Paint this component with the paintcontroller, given an on-screen region that this
    /// component exists in.
    fn paint(&self, r: &PaintController, pos: ScreenVec, size: ScreenVec);
    /// Returns a list of this component's children
    fn get_children<'a>(&'a self) -> Vec<&'a Component>;

    /// Set this components position relative to the parent. This is normally called after layout
    /// by the parent component to indicate a positioning after the child sets their size given the
    /// constraints.
    fn set_offset(&mut self, pos: ScreenVec);

    /// See set_pos() for more details.
    fn get_offset(&self) -> ScreenVec;
}
