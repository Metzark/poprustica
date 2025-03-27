use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowId}
};

pub struct Game {
    window: Option<Window>,
    width: f64,
    height: f64,
    framerate: Duration,
    last_render_time: Instant,
    mouse_position: (f64, f64)
}


impl Game {
    // Create a new game
    pub fn new(width: f64, height: f64, framerate: f64) -> Self {
        Self { 
            window: None,
            width,
            height,
            framerate: Duration::from_secs_f64(1.0 / framerate),
            last_render_time: Instant::now(),
            mouse_position: (0.0, 0.0)
        }
    }

    // Create game window tied to an event loop
    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        // Set window attributes
        let attributes = Window::default_attributes()
        .with_title("Poprustica")
        .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
        .with_resizable(false);

        // Create the window
        self.window = Some(event_loop.create_window(attributes).expect("Failed to create window"));
    }
}

// Implement the ApplicationHandler trait for Game
impl ApplicationHandler for Game {
    // Handle all window events
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        if let Some(_window) = &self.window {
            match event {
                WindowEvent::RedrawRequested => {
                    self.last_render_time += self.framerate;
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit()
                }
                WindowEvent::CursorMoved { position, ..  } => {
                    self.mouse_position = (position.x, position.y);
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    match (button, state) {
                        (MouseButton::Left, ElementState::Pressed) => {
                            println!("Left mouse button pressed!");
                        }
                        (MouseButton::Left, ElementState::Released) => {
                            println!("Left mouse button released!");
                        }
                        (MouseButton::Right, ElementState::Pressed) => {
                            println!("Right mouse button pressed!");
                        }
                        (MouseButton::Right, ElementState::Released) => {
                            println!("Right mouse button released!");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        
    }

    // Create window on resume (only fired on start)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Ensure that a window is only created once (just in case)
        if self.window.is_none() {
            self.create_window(event_loop);
        }
        // Kickstart the frame rendering loop
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.last_render_time + self.framerate));
    }

    // Handle new events
    fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: StartCause) {
        // Get not
        let now = Instant::now();

        // If its time to render a new frame, call request_redraw
        if now.duration_since(self.last_render_time) >= self.framerate {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        // Wait until time to next render or another event arrives (helps prevent cpu from getting slammed)
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.last_render_time + self.framerate));
    }
}