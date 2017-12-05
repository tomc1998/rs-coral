/// Layout constraints.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Constraints {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
}

impl Constraints {
    pub fn new(min_x: u32, min_y: u32, max_x: u32, max_y: u32) -> Constraints {
        Constraints {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}
