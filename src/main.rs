mod client;

use winit::event_loop::EventLoop;

use client::game::Game;


fn main() {
    // Create the event loop
    let event_loop: EventLoop<()> = EventLoop::new().expect("Failed to create event loop");

    // Create the game
    let mut game: Game = Game::new();

    // Run the game using the event loop
    event_loop.run_app(&mut game).expect("Failed to run app");
}
