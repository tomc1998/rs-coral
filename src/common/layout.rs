/// Layout constraints.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Constraints {
    pub min_w: u32,
    pub min_h: u32,
    pub max_w: u32,
    pub max_h: u32,
}

impl Constraints {
    pub fn new(min_w: u32, min_h: u32, max_w: u32, max_h: u32) -> Constraints {
        Constraints {
            min_w,
            min_h,
            max_w,
            max_h,
        }
    }
}
