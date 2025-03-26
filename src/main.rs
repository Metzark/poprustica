mod client;

use crate::client::{game_loop::GameLoop, shell::Shell};

// Game config stuff
const WINDOW_WIDTH: f64 = 1280.0;
const WINDOW_HEIGHT: f64 = 720.0;
const FRAMERATE: f64 = 60.0;
const TICKRATE: f64 = 30.0;

fn main() {
    // Create shell (window width and height) and event loop
    let (shell, event_loop) = Shell::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    // Create game loop (shell, framerate, tickrate)
    let mut game_loop = GameLoop::new(shell, FRAMERATE, TICKRATE);
    
    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        // Run game loop
        game_loop.run(event, control_flow);
    });
}
