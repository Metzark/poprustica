use winit::{
    event::{Event, WindowEvent, MouseButton, ElementState},
    event_loop::ControlFlow,
};
use std::time::{Duration, Instant};

use crate::client::shell::Shell;

pub struct GameLoop {
    shell: Shell,
    last_render_time: Instant,
    last_tick_time: Instant,
    framerate: Duration,
    tickrate: Duration,
    mouse_position: (f64, f64),
}

impl GameLoop {
    pub fn new(shell: Shell, framerate: f64, tickrate: f64) -> Self {
        Self {
            shell,
            last_render_time: Instant::now(),
            last_tick_time: Instant::now(),
            framerate: Duration::from_secs_f64(1.0 / framerate),
            tickrate: Duration::from_secs_f64(1.0 / tickrate),
            mouse_position: (0.0, 0.0)
        }
    }

    // Run a single loop in the game loop
    pub fn run(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, window_id } if window_id == self.shell.window.id() => {
                self.handle_window_event(event, control_flow);
            }
            Event::MainEventsCleared => {
                self.handle_update(control_flow);
            }
            Event::RedrawRequested(_) => {
                self.handle_render();
            }
            _ => {}
        }
    }

    // Handle all window events
    fn handle_window_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(_) => {
                // Keeping the window non-resizable for now...
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x, position.y);
            }
            WindowEvent::MouseInput { state, button, .. } => {
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

    // Handle updating game state (called once event loop queue is empty)
    fn handle_update(&mut self, control_flow: &mut ControlFlow) {
        // Get time right now
        let now = Instant::now();

        // Only update game state if enough time has passed since last tick
        if now.duration_since(self.last_tick_time) >= self.tickrate {
            // Update game state here
            self.last_tick_time = now;
        }

        // If elapsed time is gte the framerate, need to render a new frame
        if now.duration_since(self.last_render_time) >= self.framerate {
            // Request redraw (call to render new frame)
            self.shell.window.request_redraw();
            self.last_render_time = now;
        }

         // Schedule the next update based on the soonest needed event (tick or frame)
        *control_flow = ControlFlow::WaitUntil((self.last_tick_time + self.tickrate).min(self.last_render_time + self.framerate));
    }

    // Handle rendering a frame
    fn handle_render(&mut self) {
    }
}