extern crate coral;

use coral::common::*;
use coral::PaintController;

pub fn main() {
    let mut coral = coral::Coral::new();
    coral.config.debug_drawing = true;
    coral.start();
}
