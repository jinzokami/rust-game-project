extern crate glium;
mod render_util;

#[derive(Copy, Clone)]
struct Vert {
    position: [f32; 3],
    color: [f32; 3],
}

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    
    glium::implement_vertex!(Vert, position, color);

    let vertex_shader_src = render_util::load_ascii("res/shader.vert");
    let fragment_shader_src = render_util::load_ascii("res/shader.frag");

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let data = vec![
        Vert {position: [0.0, 0.0, 0.0], color: [0.0, 1.0, 1.0]},
        Vert {position: [1.0, 0.0, 0.0], color: [1.0, 0.0, 1.0]},
        Vert {position: [0.0, 1.0, 0.0], color: [1.0, 1.0, 0.0]},
        ];

    let buffer = glium::VertexBuffer::new(&display, &data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut closed = false;

    'main: loop {
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame.draw(&buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        frame.finish().unwrap();

        if closed {
            break 'main;
        }
    }
}