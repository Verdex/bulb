
extern crate glium;

use std::time::Instant;
use std::time::Duration;

use glium::{ Surface
           , Display
           , glutin::event_loop::EventLoop 
           , glutin::event_loop::ControlFlow
           , glutin::window::WindowBuilder
           , glutin::ContextBuilder
           };

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new();
    let context_builder = ContextBuilder::new();
    // TODO message on unwrap
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    event_loop.run( move |event, _, control_flow | {
        let next = Instant::now() + Duration::from_nanos(16_666_667); // TODO time?
        *control_flow = ControlFlow::WaitUntil(next);


        match event {

        }
    });
}
