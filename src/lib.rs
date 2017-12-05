#[macro_use]
extern crate log;
extern crate quick_gfx as qgfx;
extern crate simplelog;

mod render;
pub mod common;
pub mod component;

use common::Constraints;

pub struct Coral {
    /// The current root node in the component tree
    root: Option<Box<component::Component>>,
}

impl Coral {
    fn init_logger(&self) {
        use simplelog::{CombinedLogger, Config, LogLevelFilter, TermLogger};
        CombinedLogger::init(vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
        ]).unwrap();
    }

    /// Initialise the library.
    pub fn new() -> Coral {
        let coral = Coral { root: None };
        coral.init_logger();
        return coral;
    }

    fn relayout(&mut self, w: u32, h: u32) {
        info!("Resizing to ({}, {})", w, h);
        if self.root.is_none() {
            return;
        }
        let root = self.root.as_mut().unwrap();
        root.layout(Constraints::new(w, h, w, h));
    }

    /// Starts the application. This will initialise an OpenGL context, and block until the
    /// application closes.
    pub fn start(&mut self) {
        info!("Starting coral...");
        let mut closed = false;
        let mut g = qgfx::QGFX::new();
        // We're just re-rendering for 60fps right now.
        let frame_time = std::time::Duration::from_millis(17);
        while !closed {
            let start_frame_time = std::time::SystemTime::now();
            g.poll_events(|ev| match ev {
                qgfx::Event::WindowEvent {
                    event: ev,
                    window_id: _,
                } => match ev {
                    // Poll events to check if window has been closed
                    qgfx::WindowEvent::Closed => closed = true,
                    qgfx::WindowEvent::Resized(w, h) => {
                        self.relayout(w, h);
                    }
                    _ => (),
                },
                _ => (),
            });
            g.render();
            let elapsed = start_frame_time.elapsed().unwrap();
            if frame_time > elapsed {
                let time_to_sleep = frame_time - elapsed;
                std::thread::sleep(time_to_sleep);
            }
        }
    }
}
