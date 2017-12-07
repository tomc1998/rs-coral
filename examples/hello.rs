extern crate coral;

pub fn main() {
    let mut coral = coral::Coral::new();
    coral.config.debug_drawing = true;
    coral.start();
}
