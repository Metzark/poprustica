mod client;

use crate::client::{game_loop::GameLoop, shell::Shell};
fn main() {
    // Create shell (window width and height) and event loop
    let (shell, event_loop) = Shell::new(1280.0, 720.0);

    // Create game loop (shell, framerate, tickrate)
    let mut game_loop = GameLoop::new(shell, 5.0, 5.0);
    
    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        // Run game loop
        game_loop.run(event, control_flow);
    });
}
