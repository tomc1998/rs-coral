use std::default::Default;

/// A structure containing all configurable options.
///
/// # Important note
/// Some items in this config won't be applied if set during runtime. If this is the case, then the
/// documentation for that variable will say that explicitly.
pub struct Config {
    /// When set to true coloured rectangles will be drawn. This is to debug component layouts.
    pub debug_drawing: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            debug_drawing: false,
        }
    }
}
