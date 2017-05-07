#[macro_use]
extern crate glium;

use std::io::prelude::*;
use std::fs::File;

use glium::backend::Facade;
use glium::glutin::{Event, VirtualKeyCode, ElementState};

// todo
// add following
// uniform vec3      iResolution;           // viewport resolution (in pixels)
// uniform float     iGlobalTime;           // shader playback time (in seconds)
// uniform float     iTimeDelta;            // render time (in seconds)
// uniform int       iFrame;                // shader playback frame

fn load_shader(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut shader = String::new();
    f.read_to_string(&mut shader).unwrap();

    shader
}

fn compile_shaders(display: &Facade) -> Result<glium::Program, glium::ProgramCreationError>  {

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = load_shader("shaders/dumb.frag");

    let program = glium::Program::from_source(display, vertex_shader_src,
                                              &fragment_shader_src, None);
    program
}

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-1.0,  1.0] }; // NW
    let vertex2 = Vertex { position: [-1.0, -1.0] }; // SW

    let vertex3 = Vertex { position: [ 1.0,  1.0] }; // NE
    let vertex4 = Vertex { position: [ 1.0, -1.0] }; // SE

    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut program = compile_shaders(&display).unwrap();

    loop {
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();

        let uniforms = uniform! {
            dimensions: [width, height], // uvec2
        };

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Q)) => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::F5)) => {
                    match compile_shaders(&display) {
                        Ok(p) => program = p,
                        Err(e) => println!("Error: {}", e),
                    }
                },
                _ => ()
            }
        }
    }
}
