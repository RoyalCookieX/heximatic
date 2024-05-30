use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

struct Application;

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("hello, heximatic!");
        event_loop.exit();
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: WindowEvent,
    ) {
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("EventLoop::new");
    event_loop
        .run_app(&mut Application)
        .expect("EventLoop::run_app");
}
