mod kmath;
mod kimg;
mod video;
mod texture_buffer;
mod renderers;
mod kapp;
mod gui;

use crate::kapp::*;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut application = Application::new(&event_loop);
    
    event_loop.run(move |event, _, _| {
        application.handle_event(event);
    });
}