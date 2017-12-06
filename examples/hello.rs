extern crate coral;

use coral::common::*;
use coral::Component;
use coral::PaintController;

pub struct Center {
    pub size: ScreenVec,
    pub pos: ScreenVec,
    pub child: Box<Component>,
}

impl Center {
    pub fn new(child: Box<Component>) -> Center {
        Center {
            pos: ScreenVec::new(0,0),
            size: ScreenVec::new(0,0),
            child: child,
        }
    }
}

impl Component for Center {
    fn get_size(&self) -> ScreenVec { self.size }
    fn layout(&mut self, constraints: Constraints) {
        self.size = ScreenVec::new(constraints.max_w as i32, constraints.max_h as i32);
        self.child.layout(constraints);
        let size = self.child.get_size();
        self.child.set_offset(ScreenVec::new(self.size.x / 2 - size.x / 2, 
                                             self.size.y / 2 - size.y / 2));
    }
    fn paint(&self, r: &PaintController, pos: ScreenVec, size: ScreenVec) {
    }
    fn get_children<'a>(&'a self) -> Vec<&'a Component> {
        vec![self.child.as_ref()]
    }
    fn set_offset(&mut self, pos: ScreenVec) { self.pos = pos; }
    fn get_offset(&self) -> ScreenVec { self.pos }
}

pub struct MyComponent {
    pub size: ScreenVec,
    pub pos: ScreenVec,
    pub color: Color,
}

impl MyComponent {
    pub fn new(color: Color) -> MyComponent {
        MyComponent {
            pos: ScreenVec::new(0,0),
            size: ScreenVec::new(0,0),
            color: color,
        }
    }
}

impl Component for MyComponent {
    fn get_size(&self) -> ScreenVec { self.size }
    fn layout(&mut self, constraints: Constraints) {
        self.size.x = (constraints.max_w / 2) as i32;
        self.size.y = (constraints.max_h / 2) as i32;
    }
    fn paint(&self, r: &PaintController, pos: ScreenVec, size: ScreenVec) {
    }
    fn get_children<'a>(&'a self) -> Vec<&'a Component> { Vec::new() }
    fn set_offset(&mut self, pos: ScreenVec) { self.pos = pos; }
    fn get_offset(&self) -> ScreenVec { self.pos }
}


pub fn main() {
    let mut coral = coral::Coral::new();
    coral.config.debug_drawing = true;
    coral.set_root(Box::new(Center::new(Box::new(MyComponent::new(Color::new(1.0, 0.0, 0.0, 1.0))))));
    coral.start();
}
