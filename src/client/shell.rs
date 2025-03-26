use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};

/// Game shell containing the game window
pub struct Shell {
    pub window: winit::window::Window,
}

impl Shell {
    /// Returns a new game shell containing a window and an event loop attached to the window
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


