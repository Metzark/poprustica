use std::{
    time::{Duration, Instant},
    sync::Arc
};

use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowId}
};

use crate::client::grafx::Grafx;
use crate::client::config;

pub struct Game {
    grafx: Option<Grafx>,
    framerate: Duration,
    last_render_time: Instant,
    mouse_position: (f64, f64)
}


impl Game {
    // Create a new game
    pub fn new() -> Self {
        Self { 
            grafx: None,
            framerate: Duration::from_secs_f64(1.0 / config::FRAMERATE),
            last_render_time: Instant::now(),
            mouse_position: (0.0, 0.0)
        }
    }

    // Create game window tied to an event loop
    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        // Set window attributes
        let attributes = Window::default_attributes()
        .with_title("Poprustica")
        .with_inner_size(winit::dpi::LogicalSize::new(config::WINDOW_WIDTH, config::WINDOW_HEIGHT))
        .with_resizable(false);

        // Create the window
        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("Failed to create window"),
        );

        let grafx = pollster::block_on(Grafx::new(window.clone()));
        self.grafx = Some(grafx);
    }

}

// Implement the ApplicationHandler trait for Game
impl ApplicationHandler for Game {
    // Handle all window events
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                self.last_render_time += self.framerate;

                match &mut self.grafx {
                    Some(grafx) => {
                        let _ = grafx.render();
                    }
                    _ => {}
                }
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

    // Create window on resume (only fired on start)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Ensure that a window is only created once (just in case)
        if self.grafx.is_none() {
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
            self.grafx.as_mut().unwrap().get_window().request_redraw();
        }

        // Wait until time to next render or another event arrives (helps prevent cpu from getting slammed)
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.last_render_time + self.framerate));
    }
}