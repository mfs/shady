#[macro_use]
extern crate glium;
extern crate notify;
extern crate time;

mod shaders;

use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use glium::backend::Facade;
use glium::glutin::event::{ElementState, Event, MouseButton};
use glium::Surface;

fn load_shader(filename: &Path) -> String {
    let mut f = File::open(filename).unwrap();
    let mut shader = String::new();
    f.read_to_string(&mut shader).unwrap();

    shader
}

fn compile_shaders(
    display: &dyn Facade,
    filename: &Path,
) -> Result<glium::Program, glium::ProgramCreationError> {
    let fragment_shader_src = load_shader(filename);

    let program = glium::Program::from_source(
        display,
        shaders::VERTEX_SHADER_SRC,
        &fragment_shader_src,
        None,
    );
    program
}

fn safe_compile_shaders(program: &mut glium::Program, display: &dyn Facade, filename: &Path) {
    match compile_shaders(display, filename) {
        Ok(p) => *program = p,
        Err(e) => println!("shady: error: {}", e),
    }
}

fn init_compile_shaders(display: &dyn Facade, filename: &Path) -> glium::Program {
    match compile_shaders(display, filename) {
        Ok(p) => return p,
        Err(e) => println!("shady: error: {}", e),
    }

    let program = glium::Program::from_source(
        display,
        shaders::VERTEX_SHADER_SRC,
        shaders::DEFAULT_FRAGMENT_SHADER_SRC,
        None,
    );

    match program {
        Ok(p) => return p,
        Err(e) => {
            println!("shady: error: {}", e);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: shady <SHADER>");
        std::process::exit(1);
    }

    let file_path = match Path::new(&args[1]).canonicalize() {
        Ok(f) => f,
        Err(e) => {
            println!("shady: error: {}", e);
            std::process::exit(1);
        }
    };

    let directory = match file_path.parent() {
        Some(x) => x,
        None => {
            println!("shady: error: no parent directory");
            std::process::exit(1)
        }
    };

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex {
        position: [-1.0, 1.0],
    }; // NW
    let vertex2 = Vertex {
        position: [-1.0, -1.0],
    }; // SW

    let vertex3 = Vertex {
        position: [1.0, 1.0],
    }; // NE
    let vertex4 = Vertex {
        position: [1.0, -1.0],
    }; // SE

    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let wb = glium::glutin::window::WindowBuilder::new();
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let cb = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let mut program = init_compile_shaders(&display, &file_path);

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    watcher
        .watch(directory, RecursiveMode::NonRecursive)
        .unwrap();

    let mut frame: i32 = 0;
    let start_time: f64 = (time::OffsetDateTime::now_utc() - time::OffsetDateTime::unix_epoch())
        .as_seconds_f64()
        * 1000.0;
    let mut last_time: f64 = start_time;

    // mouse pixel coords. xy: current (if MLB down), zw: click
    let mut mouse: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    let mut mouse_tracking = false;
    let mut mouse_coord = [0, 0];

    event_loop.run(move |ev, _, cf| {
        match rx.try_recv() {
            Ok(DebouncedEvent::Create(x)) | Ok(DebouncedEvent::Write(x)) => {
                if x == file_path {
                    safe_compile_shaders(&mut program, &display, &file_path);
                }
            }
            Ok(_) => {}
            Err(_) => {}
        }

        let mut target = display.draw();
        let (width, height) = target.get_dimensions();

        let current_time: f64 = (time::OffsetDateTime::now_utc() - time::OffsetDateTime::unix_epoch())
            .as_seconds_f64()
            * 1000.0;

        let uniforms = uniform! {
            iFrame: frame, // int
            iMouse: mouse, // vec4
            iResolution: [width, height], // uvec2
            iGlobalTime: (current_time - start_time) as f32, // float
            iTimeDelta: (current_time - last_time) as f32, // float
        };

        last_time = current_time;

        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        use glium::glutin::event::WindowEvent;
        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested | WindowEvent::ReceivedCharacter('q') => {
                    *cf = glium::glutin::event_loop::ControlFlow::Exit
                }
                WindowEvent::ReceivedCharacter('r') => {
                    safe_compile_shaders(&mut program, &display, &file_path)
                }
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left,
                    ..
                } => {
                    mouse_tracking = true;
                    mouse[2] = mouse_coord[0] as f32;
                    mouse[3] = mouse_coord[1] as f32;
                }
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    button: MouseButton::Left,
                    ..
                } => {
                    mouse_tracking = false;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    mouse_coord = [position.x as i32, position.y as i32];
                    if mouse_tracking {
                        mouse[0] = position.x as f32;
                        mouse[1] = position.y as f32;
                    }
                }
                _ => {}
            },
            _ => (),
        }
        frame += 1;
    });
}
