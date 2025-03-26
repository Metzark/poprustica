use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct Shell {
    pub window: winit::window::Window,
}

impl Shell {
    pub fn new(width: f64, height: f64) -> (Self, EventLoop<()>) {
        // Create event loop
        let event_loop: EventLoop<()> = EventLoop::new();

        // Create window
        let window: winit::window::Window = WindowBuilder::new()
            .with_title("Poprustica")
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .with_resizable(false)
            .build(&event_loop)
            .expect("Failed to create window");

        // Return the new shell and event loop
        (Self { window }, event_loop)
    }
}


