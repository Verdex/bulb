
#[macro_use]
extern crate glium;

use std::time::Instant;
use std::time::Duration;

use glium::{ Surface
           , Display
           , Program
           , vertex::VertexBuffer
           , IndexBuffer
           , index::PrimitiveType
           , texture::RawImage2d
           , texture::Texture2d
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
out vec2 v_texture_coords;

uniform mat4 aspect;

void main() {
    v_texture_coords = texture_coords;
    gl_Position = aspect * vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADDER : &'static str = r#"
#version 140

in vec2 v_texture_coords;
out vec4 color;

uniform sampler2D text;

void main() {
    color = texture(text, v_texture_coords);
}
"#;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new();
    let context_builder = ContextBuilder::new();
    // TODO message on unwrap
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    // TODO message on unwrap
    let screen_vertices = VertexBuffer::new(&display, &[
        Vertex{ position: [1.0, 1.0], texture_coords: [1.0, 1.0] }, 
        Vertex{ position: [-1.0, 1.0], texture_coords: [0.0, 1.0] },
        Vertex{ position: [-1.0, -1.0], texture_coords: [0.0, 0.0] },
        Vertex{ position: [1.0, -1.0], texture_coords: [1.0, 0.0] },
    ]).unwrap();

    let indices = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[
        0u16, 2, 3,
        0, 1, 2
    ]).unwrap();

    let program = Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADDER, None).unwrap();

    // TODO into loop later
    let image = RawImage2d::from_raw_rgba_reversed(&[0u8, 200, 0, 0,
                                                     0, 200, 200, 0,
                                                     0, 0, 200, 0,
                                                     0, 0, 0, 0 ], (2, 2));
    let texture = Texture2d::new(&display, image).unwrap(); // TODO 

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

        let mut target = display.draw();

        let aspect = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
        
            [
                [aspect_ratio, 0.0, 0.0, 0.0],
                [0.0,          1.0, 0.0, 0.0],
                [0.0,          0.0, 1.0, 0.0],
                [0.0,          0.0, 0.0, 1.0],
            ]
        };

        let uniforms = uniform! {
            aspect: aspect,
            text: &texture,
        };

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&screen_vertices, 
                    &indices, 
                    &program, 
                    &uniforms, 
                    &Default::default()).unwrap();// TODO unwrap
        target.finish().unwrap(); // TODO unwrap
    });
}
