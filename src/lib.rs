#[macro_use]
extern crate log;
extern crate quick_gfx as qgfx;
extern crate simplelog;

extern crate specs;

mod config;
mod render;
pub mod common;
pub mod entity;
pub mod utils;

pub use config::Config;
pub use render::Controller as PaintController;

use entity::Entity;
use common::{Constraints, ScreenVec};

pub struct Coral {
    /// The current root node in the entity tree
    root: Option<Entity>,
    window_size: ScreenVec,
    world: specs::World,
    pub config: Config,
}

impl Coral {
    fn init_logger(&self) {
        use simplelog::{CombinedLogger, Config, LogLevelFilter, TermLogger};
        CombinedLogger::init(vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        ]).unwrap();
    }

    fn setup_world(&mut self) {
        self.world.register::<entity::ChildrenComponent>();
        self.world.register::<entity::LayoutComponent>();
    }

    /// Initialise the library.
    pub fn new() -> Coral {
        let mut coral = Coral {
            root: None,
            window_size: ScreenVec::new(0, 0),
            config: Default::default(),
            world: specs::World::new(),
        };
        coral.setup_world();
        coral.init_logger();
        return coral;
    }

    fn relayout(&self) {
        if self.root.is_none() {
            return;
        }
        let root = self.root.unwrap();
        let (w, h) = (self.window_size.x as u32, self.window_size.y as u32);
        info!("Resizing to ({}, {})", w, h);
        entity::layout(root, Constraints::new(w, h, w, h), &self.world);
    }

    fn repaint(&self, g: &mut qgfx::QGFX) {
        if self.root.is_none() {
            return;
        }
        info!("Repainting");
        let root = self.root.as_ref().unwrap();
        let controller = render::Controller::new(g.get_renderer_controller());
        if self.config.debug_drawing {
            //render::debug_render(&controller, root);
        }
        else {
            //root.paint(&controller, ScreenVec::new(0, 0), self.window_size);
        }
    }

    /// Perform a full layout / paint / rasterize update
    fn layout_paint_render(&mut self, g: &mut qgfx::QGFX) {
        self.relayout();
        self.repaint(g);
        g.recv_data();
        g.render();
    }

    pub fn set_root(&mut self, root: Entity) {
        self.root = Some(root);
    }

    /// Starts the application. This will initialise an OpenGL context, and block until the
    /// application closes.
    pub fn start(&mut self) {
        info!("Starting coral...");
        self.relayout();
        let mut closed = false;
        let mut g = qgfx::QGFX::new();
        // We're just re-rendering for 60fps right now.
        let frame_time = std::time::Duration::from_millis(17);
        while !closed {
            let start_frame_time = std::time::SystemTime::now();
            let mut needs_repaint = false;
            g.poll_events(|ev| match ev {
                qgfx::Event::WindowEvent {
                    event: ev,
                    window_id: _,
                } => match ev {
                    // Poll events to check if window has been closed
                    qgfx::WindowEvent::Closed => closed = true,
                    qgfx::WindowEvent::Resized(w, h) => {
                        self.window_size = ScreenVec::new(w as i32, h as i32);
                        needs_repaint = true;
                    }
                    _ => (),
                },
                _ => (),
            });

            if needs_repaint { self.layout_paint_render(&mut g); }

            let elapsed = start_frame_time.elapsed().unwrap();
            if frame_time > elapsed {
                let time_to_sleep = frame_time - elapsed;
                std::thread::sleep(time_to_sleep);
            }
        }
    }
}
