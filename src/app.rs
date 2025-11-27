use winit::event_loop::ActiveEventLoop;
use crate::{trait_field, trait_field_ref};

pub trait Window {
    fn start(&mut self, event_loop: &ActiveEventLoop);
    fn draw(&mut self, event_loop: &ActiveEventLoop);
    fn exit(&mut self, event_loop: &ActiveEventLoop);

    trait_field!(x: i32);
    trait_field_ref!(y: i32);
}