use std::ops::{Deref, DerefMut};
use qgfx::RendererController;

/// Currently a wrapper around the qgfx controller. Just allowing for different graphical backends
/// in the future.
#[derive(Clone)]
pub struct Controller<'a>(RendererController<'a>);

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
