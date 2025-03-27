use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler, event::{StartCause, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow}, window::{Window, WindowId}
};

pub struct App {
    window: Option<Window>,
    width: f64,
    height: f64,
    framerate: Duration,
    tickrate: Duration,
    last_render_time: Instant,
    last_tick_time: Instant
}


impl App {
    pub fn new(width: f64, height: f64, framerate: f64, tickrate: f64) -> Self {
        Self { 
            window: None,
            width,
            height,
            framerate: Duration::from_secs_f64(1.0 / framerate),
            tickrate: Duration::from_secs_f64(1.0 / tickrate),
            last_render_time: Instant::now(),
            last_tick_time: Instant::now()
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

impl ApplicationHandler for App {
    // Handle all window events
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        if let Some(window) = &self.window {
            match event {
                WindowEvent::RedrawRequested => {
                    println!("Draw at {:?}", Instant::now());
                    self.last_render_time += self.framerate;
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit()
                }
                WindowEvent::CursorMoved { device_id, position } => {
                    // self.mouse_position = (position.x, position.y);
                    println!("mouse moved");
                }
                _ => {}
            }
        }
        
    }

    // Create window on resume
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.create_window(event_loop);
        }
        // Kickstart the loop
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            self.last_render_time + self.framerate,
        ));
    }
    fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: StartCause) {
        let now = Instant::now();
        if now.duration_since(self.last_render_time) >= self.framerate {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(self.last_render_time + self.framerate));
    }
}