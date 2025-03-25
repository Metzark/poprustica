use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct Shell {
    pub window: winit::window::Window,
    pub event_loop: EventLoop<()>,
}

impl Shell {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Poprustica")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .expect("Failed to create window");

        Self { window, event_loop }
    }
}


