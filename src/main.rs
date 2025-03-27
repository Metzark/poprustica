mod client;

use winit::event_loop::EventLoop;

use client::game::Game;


// Game config stuff
const WINDOW_WIDTH: f64 = 1280.0;
const WINDOW_HEIGHT: f64 = 720.0;
const FRAMERATE: f64 = 30.0;

fn main() {
    // Create the event loop
    let event_loop: EventLoop<()> = EventLoop::new().expect("Failed to create event loop");

    // Create the game
    let mut game: Game = Game::new(WINDOW_WIDTH, WINDOW_HEIGHT, FRAMERATE);

    // Run the game using the event loop
    event_loop.run_app(&mut game).expect("Failed to run app");
}
