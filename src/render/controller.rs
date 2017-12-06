use std::ops::{Deref, DerefMut};
use std::boxed::Box;
use qgfx::RendererController;

/// Currently a wrapper around the qgfx controller. Just allowing for different graphical backends
/// in the future.
#[derive(Clone)]
pub struct Controller<'a>(Box<RendererController<'a>>);

impl<'a> Controller<'a> {
    pub fn new(c: Box<RendererController<'a>>) -> Controller<'a> {
        Controller(c)
    }
}

impl<'a> Deref for Controller<'a> {
    type Target = RendererController<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Controller<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
