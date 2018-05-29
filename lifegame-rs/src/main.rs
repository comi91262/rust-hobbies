#[macro_use]
extern crate glium;

mod square;

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

        uniform mat4 matrix;

        void main() {
            color = matrix * vec4(0.0, 1.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = 1.0;
    let mut closed = false;
    let mut position_x = 0.0;
    let mut position_y = 0.0;

    let mut fields = [false; 20 * 20];

    while !closed {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, if fields[((position_y / 60.0) as u32 * 20 + (position_x / 60.0) as u32) as usize] {
                    println!("matrix x {:?}", position_x);
                    println!("matrix y {:?}", position_y);
                    0.0
                } else {
                    1.0f32
                }, 0.0, 0.0],
                
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&positions, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::CursorMoved {position: (x, y), ..}  => {
                        position_x = if x > 1200.0 { 1200.0 - 1.0 } else if x < 0.0 { 0.0 } else { x };
                        position_y = if y > 1200.0 { 1200.0 - 1.0 } else if y < 0.0 { 0.0 } else { y };
                    },
                    glutin::WindowEvent::MouseInput {state, ..}  => {
                        match state {
                            glutin::ElementState::Pressed => {
                                let x = (position_x / 60.0) as u32;
                                let y = (position_y / 60.0) as u32;
                                println!("{:?}", x);
                                println!("{:?}", y);
                                println!("{:?}", state);
                                println!("{:?}", fields[(y * 20 + x) as usize]);
                                fields[(y * 20 + x) as usize] = true; 
                                
                            }
                            glutin::ElementState::Released => println!("{:?}", state), 
                        };
                    },
                    _ => ()
                },
                _ => (),
            }
        });
    }
}
