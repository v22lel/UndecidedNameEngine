use crate::app::Window;
use winit::application::ApplicationHandler;
use winit::error::{EventLoopError, OsError};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

#[derive(Debug)]
pub enum WindowError {
    EventLoop(EventLoopError),
    Os(OsError)
}

struct WindowInstance {
    handle: winit::window::Window,
    window: Box<dyn Window>,
}

pub struct WindowManager {
    raw_handles: Vec<WindowInstance>,
}

impl<'a> ApplicationHandler for WindowManager {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let test = WindowInstance {};
        
        test.window.get_y_mut()
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.raw_handles.retain(|x| x.handle.id() != window_id);

                if self.raw_handles.is_empty() {
                    event_loop.exit();
                }
            }
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(_) => {}
            WindowEvent::Moved(_) => {}
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::PinchGesture { .. } => {}
            WindowEvent::PanGesture { .. } => {}
            WindowEvent::DoubleTapGesture { .. } => {}
            WindowEvent::RotationGesture { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {

            }
        }
    }
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            raw_handles: vec![]
        }
    }

    pub fn start(&mut self) -> Result<(), WindowError> {
        let event_loop = EventLoop::new().map_err(WindowError::EventLoop)?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).map_err(WindowError::EventLoop)
    }

    pub fn open_new_window(&mut self, window: impl Window + 'static) -> Result<(), WindowError> {
        let window = event_loop.create_window(winit::window::Window::default_attributes())
            .map_err(WindowError::Os)?;

        self.raw_handles.push(window);
        Ok(())
    }
}
