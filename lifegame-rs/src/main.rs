#[macro_use]
extern crate glium;

mod squares;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(600, 600)
        .with_title("lifegame");

    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;

        uniform float t;

        void main() {
           vec2 pos = position;
           gl_Position = vec4(pos, 0.0, t);
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

    let mut t: u32 = 1;
    let mut closed = false;
    let mut position_x = 0.0;
    let mut position_y = 0.0;

    let mut fields = [false; 512];
    let mut squares: [glium::VertexBuffer<squares::Vertex>; 20 * 20];

    for i in 0..400 {
        squares[i] = a(&display);
    }


    //for y in 0..20 {
    //    for x in 0..20 {
    //        let positions = glium::VertexBuffer::new(&display, &[squares::VERTICES[i], squares::VERTICES[i], squares::VERTICES[i]]).unwrap();
    //        let indices = glium::index::PrimitiveType
    //            IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
    //                             &squares::INDICES).unwrap();
    //    }
    //}
    //
    
    //0 0  (-1.0, 1.0), (-1.0, 0.9), (-0.9, 1.0) (-0.9, -0.9)
    //1 0  (-0.9, 1.0), (-0.9, 0.9), (-0.8, 1.0) (-0.8, -0.9)



    //let positions1 =
    let positions2 = glium::VertexBuffer::new(&display, &[squares::VERTICES[1], squares::VERTICES[2], squares::VERTICES[22], squares::VERTICES[23]]).unwrap();
    //let positions2 = glium::VertexBuffer::new(&display, &[squares::VERTICES[0], squares::VERTICES[21], squares::VERTICES[22]]).unwrap();
    //let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
     let indices =
        glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0u16, 1u16, 3u16, 0u16, 2u16, 3u16]).unwrap();
    //let buffer = glium::uniforms::UniformBuffer::new(&display, fields).unwrap();

    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&squares[0], &indices, &program, &uniform!{t: if fields[0] { 1.0f32} else { 0.0f32} },
                    &Default::default()).unwrap();
        target.draw(&positions2, &indices, &program, &uniform!{t: if fields[1] { 1.0f32} else { 0.0f32} },
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

fn a(display: &glium::Display) -> glium::VertexBuffer<squares::Vertex> {
 glium::VertexBuffer::new(&display, &[squares::VERTICES[0], squares::VERTICES[1], squares::VERTICES[21], squares::VERTICES[22]]).unwrap()
}
