mod client;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use std::time::{Duration, Instant};

use crate::client::shell::Shell;
fn main() {
    // Create game shell
    let shell: Shell = Shell::new();

    let mut last_render_time: Instant = Instant::now();
    let target_frametime: Duration = Duration::from_secs_f64(1.0 / 60.0); // FPS

    let mut mouse_position: (f64, f64) = (0.0, 0.0);
    
    // Run the event loop
    shell.event_loop.run(move |event: Event<'_, ()>, _, control_flow: &mut ControlFlow| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == shell.window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(_) => {
                        // Handle window resize here (will be needed for wgpu later)
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        // Track mouse position
                        mouse_position = (position.x, position.y);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                // Update game logic here
                
                // Request redraw after updates
                shell.window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let now: Instant = Instant::now();
                // Calculate delta time for frame timing
                let _dt: Duration = now - last_render_time;
                last_render_time = now;

                println!("Mouse position: ({}, {})", mouse_position.0, mouse_position.1);
                                
                // Schedule next frame
                *control_flow = ControlFlow::WaitUntil(now + target_frametime);
            }
            _ => {}
        }
    });
}
