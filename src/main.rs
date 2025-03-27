mod client;

use winit::event_loop::EventLoop;

use client::app::App;


// Game config stuff
const WINDOW_WIDTH: f64 = 1280.0;
const WINDOW_HEIGHT: f64 = 720.0;
const FRAMERATE: f64 = 5.0;
const TICKRATE: f64 = 5.0;

fn main() {

    let event_loop: EventLoop<()> = EventLoop::new().expect("Failed to create event loop");

    let mut app: App = App::new(WINDOW_WIDTH, WINDOW_HEIGHT, FRAMERATE, TICKRATE);

    event_loop.run_app(&mut app).expect("Failed to run app");
}
