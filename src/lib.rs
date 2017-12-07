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

use entity::{Entity, LayoutComponent, ChildrenComponent, LayoutStrategy};
use common::{Constraints, ScreenVec};

/// A list of systems used by the Coral instance.
#[derive(Clone)]
struct Systems {
    layout_system: entity::LayoutSystem,
}
impl Systems {
    fn new() -> Systems {
        Systems {
            layout_system: entity::LayoutSystem::new(),
        }
    }
}

pub struct Coral {
    /// The current root node in the entity tree
    root: Option<Entity>,
    /// The layout system being used
    window_size: ScreenVec,
    systems: Systems,
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
            systems: Systems::new(),
            config: Default::default(),
            world: specs::World::new(),
        };
        coral.setup_world();
        coral.init_logger();
        return coral;
    }

    /// Perform a full layout / paint / rasterize update
    fn layout_paint_render(&mut self, g: &mut qgfx::QGFX) {
        if self.root.is_none() {
            warn!("Attempting to re-render, but no root node specified!")
        }
        else {
            // Setup systems for layout + drawing
            // Layout system:
            let (w, h) = (self.window_size.x as u32, self.window_size.y as u32);
            self.systems.layout_system.constraints = Constraints::new(w, h, w, h);
            let mut dispatcher_builder = specs::DispatcherBuilder::new()
                .add(self.systems.layout_system, "layout", &[]);

            // Paint system:
            let controller = render::Controller::new(g.get_renderer_controller());
            if self.config.debug_drawing {
                let mut debug_render_system = render::DebugRenderSystem::new();
                debug_render_system.c = Some(controller.clone());
                debug_render_system.root = self.root;
                dispatcher_builder = dispatcher_builder.add(debug_render_system, "paint", &["layout"]);
            }
            else {
                unimplemented!();
                //root.paint(&controller, ScreenVec::new(0, 0), self.window_size);
            }

            // Run it all
            dispatcher_builder.build().dispatch(&self.world.res);
            self.world.maintain();
        }
        g.recv_data();
        g.render();
    }

    pub fn set_root(&mut self, root: Entity) {
        self.root = Some(root);
        self.systems.layout_system.root = Some(root);
    }

    /// Create an entity and add it to the world. This will NOT trigger a redraw - this must be
    /// done manually! Otherwise, adding multiple children to the world would result in a bad state
    /// whilst laying out.
    /// 
    /// # Params
    /// * `parent` - The parent of this entity. If None, this becomes a root node - see set_root.
    /// * `layout` - The layout strategy to use for this component.
    pub fn create_entity(&mut self,
                         parent: Option<Entity>,
                         layout: LayoutStrategy) -> Entity {
        // First, we need to build the entity
        let entity = self.world.create_entity()
            .with(LayoutComponent::new(layout))
            .with(ChildrenComponent::new())
            .build();

        // Now, find the parent (if any) and add this entity as a child
        if parent.is_some() {
            let parent = parent.unwrap();
            let mut children_storage = self.world.write();
            let children : &mut ChildrenComponent = children_storage.get_mut(parent)
                .expect("Tried to create a child entity, but parent entity had no children
                        component");
            children.children.push(entity);
        }

        // Finally, return the new entity
        return entity;
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
                    qgfx::WindowEvent::Refresh => needs_repaint = true,
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
