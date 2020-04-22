
#[macro_use]
extern crate glium;

use std::time::Instant;
use std::time::Duration;

use glium::{ Surface
           , Display
           , glutin::event_loop::EventLoop 
           , glutin::event_loop::ControlFlow
           , glutin::event::Event
           , glutin::event::WindowEvent
           , glutin::window::WindowBuilder
           , glutin::ContextBuilder
           };

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
    texture_coords: [f32; 2],
}

implement_vertex!(Vertex, position, texture_coords);

const VERTEX_SHADER : &'static str = r#"
#version 140

in vec2 position;
in vec2 texture_coords;
out vec2 v_tex_coords;

uniform mat4 matrix;

void main() {
    v_tex_coords = texture_coords;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADDER : &'static str = r#"
#version 140

in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords);
}
"#;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new();
    let context_builder = ContextBuilder::new();
    // TODO message on unwrap
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    event_loop.run( move |event, _, control_flow | {
        let next = Instant::now() + Duration::from_nanos(16_666_667); // TODO time?
        *control_flow = ControlFlow::WaitUntil(next);


        // TODO there's different scenarios that destory a loop
        // should handle basically the same tear down actions in
        // any given one of them.
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    WindowEvent::Destroyed=> {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    /*
                     Resized(PhysicalSize<u32>),
                     DroppedFile(PathBuf),
                     KeyboardInput {
                        device_id: DeviceId,
                        input: KeyboardInput,
                    },
                    MouseWheel {
                        device_id: DeviceId,
                        delta: MouseScrollDelta,
                        phase: TouchPhase,
                    },
                     MouseInput {
                        device_id: DeviceId,
                        state: ElementState,
                        button: MouseButton,
                    },
                    */
                    _ => (),
                }
            },
            Event::LoopDestroyed => {
                *control_flow = ControlFlow::Exit;
                return;
            },
            _ => (),
        }
    });
}
