#[macro_use]
extern crate glium;

mod square;

fn make_square() {
}


fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(600, 600)
        .with_title("lifegame");

    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &square::VERTICES).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &square::INDICES).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(0.0, 1.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&positions, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        // we update `t`
        //        t += 0.0002;
        //        if t > 0.5 {
        //            t = -0.5;
        //        }
        //
        //        let mut target = display.draw();
        //        target.clear_color(0.0, 0.0, 1.0, 1.0);
        //let uniforms = uniform! {
        //    matrix: [
        //        [ t.cos(), t.sin(), 0.0, 0.0],
        //        [-t.sin(), t.cos(), 0.0, 0.0],
        //        [0.0, 0.0, 1.0, 0.0],
        //        [0.0, 0.0, 0.0, 1.0f32],
        //    ]
        //};
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}
