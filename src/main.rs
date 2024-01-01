mod sf;
mod application;
mod event_loop;
mod util;

fn main() {
    event_loop::run();
}

// add a thread pool to the application
