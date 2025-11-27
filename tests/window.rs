use std::thread;
use winit::event_loop::ActiveEventLoop;
use une::app::Window;
use une::windowing::WindowManager;

fn main() {
    let mut window = WindowManager::new();
    window.start().unwrap();
}