use std::{
    time::Instant,
    sync::Arc
};

use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent, MouseButton, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{Window, WindowId}
};

use crate::client::grafx::{
    Grafx,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

pub struct Game {
    grafx: Option<Grafx>,
    mouse_position: (f64, f64)
}


impl Game {
    /// Create a new game
    pub fn new() -> Self {
        Self { 
            grafx: None,
            mouse_position: (0.0, 0.0)
        }
    }

    /// Create game window tied to an event loop
    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        // Set window attributes
        let attributes = Window::default_attributes()
        .with_title("Poprustica")
        .with_inner_size(winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
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
    /// Handle all window events
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let grafx: &mut Grafx = self.grafx.as_mut().expect("Failing retrieveing grafx on new events");
        match event {
            WindowEvent::RedrawRequested => {
                grafx.set_last_render_time(grafx.get_framerate());
                grafx.render();
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

    /// Create window on resume (only fired on start)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Ensure that a window is only created once (just in case)
        if self.grafx.is_none() {
            self.create_window(event_loop);
        }

        let grafx: &Grafx = self.grafx.as_ref().expect("Failed creating the window due to invalid grafx");

        // Kickstart the frame rendering loop
        event_loop.set_control_flow(ControlFlow::WaitUntil(grafx.get_last_render_time() + grafx.get_framerate()));
    }

    /// Handle new events
    fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: StartCause) {
        // Get not
        let now = Instant::now();


        match self.grafx.as_ref() {
            Some(grafx) => {
                // If its time to render a new frame, call request_redraw
                if now.duration_since(grafx.get_last_render_time()) >= grafx.get_framerate() {
                    grafx.get_window().request_redraw();
                }

                // Wait until time to next render or another event arrives
                event_loop.set_control_flow(ControlFlow::WaitUntil(
                    grafx.get_last_render_time() + grafx.get_framerate()
                ));
            }
            None => {
            }
        }
    }
}