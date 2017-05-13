#[macro_use]
extern crate glium;
extern crate time;

mod shaders;

use std::io::prelude::*;
use std::fs::File;

use glium::backend::Facade;
use glium::glutin::{Event, VirtualKeyCode, ElementState, MouseButton};

fn load_shader(filename: &str) -> String {
    let mut f = File::open(filename).unwrap();
    let mut shader = String::new();
    f.read_to_string(&mut shader).unwrap();

    shader
}

fn compile_shaders(display: &Facade, filename: &str) ->
    Result<glium::Program, glium::ProgramCreationError>  {

    let fragment_shader_src = load_shader(filename);

    let program = glium::Program::from_source(display, shaders::VERTEX_SHADER_SRC,
                                              &fragment_shader_src, None);
    program
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: shady <SHADER>");
        std::process::exit(1);
    }

    let ref fragment_shader = args[1];

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

    let mut program = compile_shaders(&display, fragment_shader).unwrap();

    let mut frame: i32 = 0;
    let start_time: f64 = time::precise_time_s();
    let mut last_time: f64 = start_time;

    // mouse pixel coords. xy: current (if MLB down), zw: click
    let mut mouse: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut mouse_tracking = false;
    let mut mouse_coord = [0, 0];

    loop {
        let mut target = display.draw();
        let (width, height) = target.get_dimensions();

        let current_time = time::precise_time_s();

        let uniforms = uniform! {
            iFrame: frame, // int
            iMouse: mouse, // vec4
            iResolution: [width, height], // uvec2
            iGlobalTime: (current_time - start_time) as f32, // float
            iTimeDelta: (current_time - last_time) as f32, // float
        };

        last_time = current_time;

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Q)) => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::F5)) => {
                    match compile_shaders(&display, fragment_shader) {
                        Ok(p) => program = p,
                        Err(e) => println!("Error: {}", e),
                    }
                },
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    mouse_tracking = true;
                    mouse[2] = mouse_coord[0] as f32;
                    mouse[3] = mouse_coord[1] as f32;
                },
                Event::MouseInput(ElementState::Released, MouseButton::Left) => {
                    mouse_tracking = false;
                },
                Event::MouseMoved(x, y) => {
                    mouse_coord = [x, y];
                    if mouse_tracking {
                        mouse[0] = x as f32;
                        mouse[1] = y as f32;
                    }
                },
                _ => ()
            }
        }
        frame += 1;
    }
}
